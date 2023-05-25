use bitmaps::Bitmap;
use chrono::{DateTime, FixedOffset};
use futures::{stream::BoxStream, FutureExt, StreamExt};
use google_cloud_default::WithAuthExt;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::{
        error::ErrorResponse,
        objects::{
            delete::DeleteObjectRequest,
            get::GetObjectRequest,
            list::ListObjectsRequest,
            upload::{Media, UploadObjectRequest, UploadType},
        },
    },
};
use s3s::{dto::StreamingBlob, s3_error, S3Result};

use anyhow::Result;

// TODO put them in cli parameters or a config file
const BUCKET_NAME: &str = "multipart_uploads";
const EXPIRY_TIME_SECONDS: u64 = 60 * 60 * 24 * 7; // 7 days

fn transmute_result_for_s3error<T>(
    res: Result<T, google_cloud_storage::http::Error>,
) -> S3Result<T> {
    res.map_err(|e| {
        log::error!("google cloud storage error: {:?}", e);
        s3_error!(InternalError, "internal error")
    })
}

/// a safe string is a string that is safe to use in our little macros below
/// ie, no slashes!
pub(crate) struct SafeString {
    inner: String,
}

impl SafeString {
    pub(crate) fn new(ini: String) -> Self {
        Self {
            inner: ini.replace("/", "%2F"),
        }
    }
}

impl std::fmt::Display for SafeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<String> for SafeString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// a macro to stringbuild the directory where parts will live
/// the directory is the bucket name, the object name, and the upload id
macro_rules! multipart_loc {
    ($bucket_name:expr, $object_name:expr, $upload_id:expr) => {
        format!("{}-{}-{}", $bucket_name, $object_name, $upload_id)
    };
}

pub(crate) use multipart_loc;

macro_rules! multipart_loc_with_part {
    ($bucket_name:expr, $object_name:expr, $upload_id:expr, $part_number:expr) => {
        format!(
            "{}/{}",
            multipart_loc!($bucket_name, $object_name, $upload_id),
            $part_number
        )
    };
}

pub(crate) use multipart_loc_with_part;

macro_rules! multipart_loc_with_marker {
    ($bucket_name:expr, $object_name:expr, $upload_id:expr) => {
        format!(
            "{}/marker",
            multipart_loc!($bucket_name, $object_name, $upload_id)
        )
    };
}

pub(crate) use multipart_loc_with_marker;

/// fast and memory-efficient tracker for which parts we have when we're wrapping up an upload.
pub struct PartTracker {
    inner: [bitmaps::Bitmap<1000>; 10],
    largest: u8,
}

impl PartTracker {
    fn new() -> Self {
        Self {
            inner: Default::default(),
            largest: 0,
        }
    }

    fn add_part(&mut self, part_number: u8) -> Result<()> {
        if part_number >= 10000 {
            return Err(anyhow::anyhow!("part number too large"));
        }
        self.inner[part_number as usize / 1000].set(part_number as usize % 1000, true);
        if part_number > self.largest {
            self.largest = part_number;
        }
        Ok(())
    }
    fn has_part(&self, part_number: u8) -> Result<bool> {
        if part_number >= 10000 {
            return Err(anyhow::anyhow!("part number too large"));
        }
        if part_number > self.largest {
            return Ok(false);
        }
        Ok(self.inner[part_number as usize / 1000].get(part_number as usize % 1000))
    }

    /// returns true if we're filled up through the largest part number we've seen
    /// TODO test me this is some real leetcode moment
    fn is_complete(&self) -> bool {
        let last_full = self.largest / 1000;
        let scan_fulls = self.inner.iter().take(last_full.into());
        for i in scan_fulls {
            if !i.is_full() {
                return false;
            }
        }
        // get the last bitmap that we think has indices, and check up through the largest
        let last: Bitmap<1000> = self.inner[(last_full + 1) as usize];
        let should_be_next_false = (self.largest as usize) % 1000;
        let next_false_on_last = last.next_false_index(0);
        next_false_on_last.map_or(true, |i| i > should_be_next_false)
    }
}

/// a reader that can read parts from a (complete) multipart upload
struct PartsReader<'a> {
    num_parts: u32,
    // use flatten to map the inner stream of parts to a stream of bytes
    // use next to map part numbers to
    inner_stream: BoxStream<'a, Result<bytes::Bytes>>,
}

impl<'a> PartsReader<'a> {
    async fn new(
        num_parts: u32,
        client: Client,
        bucket_name: String,
        upload_id: String,
        object_name: String,
    ) -> crate::multipart_uploads::PartsReader<'a> {
        let inner_stream = futures::stream::iter(1..=num_parts)
            .then(|part_number| {
                let req = GetObjectRequest {
                    bucket: bucket_name.clone(),
                    object: multipart_loc_with_part!(
                        bucket_name,
                        object_name,
                        upload_id,
                        part_number
                    ),
                    ..Default::default()
                };
                client
                    .download_streamed_object(&req, &Default::default())
                    .map(|res| match res {
                        Ok(stream) => stream,
                        Err(e) => futures::stream::once(async { Err(e) }),
                    })
            })
            .flat_map(|stream| stream.map_err(|e| anyhow::anyhow!(e)));
        Self {
            num_parts,
            inner_stream: Box::new(inner_stream),
        }
    }
}

pub struct CloudStorageForMultipartConstruction {
    client: Client,
}

impl CloudStorageForMultipartConstruction {
    pub async fn new() -> Self {
        let config = ClientConfig::default().with_auth().await.unwrap();
        let client = Client::new(config);

        Self { client }
    }

    /// creates a spot to put parts of a multipart upload in a cloud storage bucket
    /// marks the existence of the bucket with a creation time. eventually we'll use to clean up partial uploads.
    pub async fn create_multipart_upload_folder(
        &self,
        client_bucket_name: SafeString,
        client_object_name: SafeString,
        upload_id: SafeString,
    ) -> S3Result<()> {
        // create a folder in the cloud storage bucket for this multipart upload
        let upload_type = UploadType::Simple(Media::new(multipart_loc_with_marker!(
            client_bucket_name,
            client_object_name,
            upload_id
        )));
        let datetime = chrono::Utc::now().to_rfc3339();
        let obj = transmute_result_for_s3error(
            self.client
                .upload_object(
                    &UploadObjectRequest {
                        bucket: BUCKET_NAME.to_string(),
                        ..Default::default()
                    },
                    datetime.to_string(),
                    &upload_type,
                )
                .await,
        )?;
        Ok(())
    }

    /// checks that the upload exists in the cloud storage bucket
    pub async fn check_upload_exists(
        &self,
        client_bucket_name: SafeString,
        client_object_name: SafeString,
        upload_id: SafeString,
    ) -> S3Result<bool> {
        // check that the part_id exists in the cloud storage bucket- it should be at
        let list_object_req = ListObjectsRequest {
            bucket: BUCKET_NAME.to_string(),
            delimiter: Some("/".to_string()),
            prefix: Some(multipart_loc!(
                client_bucket_name,
                client_object_name,
                upload_id
            )),
            ..Default::default()
        };
        let list_object_resp =
            transmute_result_for_s3error(self.client.list_objects(&list_object_req).await)?;
        if let Some(items) = list_object_resp.items {
            Ok(!items.is_empty())
        } else {
            Ok(false)
        }
    }

    /// puts a part into the initiated spot for the upload
    pub async fn put_upload_part(
        &self,
        client_bucket_name: SafeString,
        client_object_name: SafeString,
        upload_id: SafeString,
        part_no: u32,
        body: StreamingBlob,
    ) -> S3Result<()> {
        // put the part in the cloud storage bucket
        // TODO ought we check it doesn't exist yet?
        let upload_type = UploadType::Simple(Media::new(multipart_loc_with_part!(
            client_bucket_name,
            client_object_name,
            upload_id,
            part_no
        )));
        let data = reqwest::Body::wrap_stream(body);
        let _ = transmute_result_for_s3error(
            self.client
                .upload_object(
                    &UploadObjectRequest {
                        bucket: BUCKET_NAME.to_string(),
                        ..Default::default()
                    },
                    data,
                    &upload_type,
                )
                .await,
        )?;
        Ok(())
    }

    /// deletes all the parts and the upload folder
    pub async fn cleanup_upload(
        &self,
        client_bucket_name: SafeString,
        client_object_name: SafeString,
        upload_id: SafeString,
    ) -> S3Result<()> {
        // list the objects in the right folder
        let list_object_req = ListObjectsRequest {
            bucket: BUCKET_NAME.to_string(),
            delimiter: Some("/".to_string()),
            prefix: Some(multipart_loc!(
                client_bucket_name,
                client_object_name,
                upload_id
            )),
            ..Default::default()
        };
        let mut list_object_resp =
            transmute_result_for_s3error(self.client.list_objects(&list_object_req).await)?;
        // TODO test this pagination behavior as you delete stuff.
        loop {
            let items = list_object_resp.items.unwrap();
            for item in items {
                let delete_request = DeleteObjectRequest {
                    bucket: BUCKET_NAME.to_string(),
                    object: item.name,
                    ..Default::default()
                };
                self.client.delete_object(&delete_request).await;
            }
            if list_object_resp.next_page_token.is_some() {
                let list_object_req = ListObjectsRequest {
                    bucket: BUCKET_NAME.to_string(),
                    page_token: list_object_resp.next_page_token,
                    ..Default::default()
                };
                list_object_resp =
                    transmute_result_for_s3error(self.client.list_objects(&list_object_req).await)?;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// removes a key and all its sub-keys from the bucket
    pub async fn rm_rf(&self, root: String) -> S3Result<()> {
        log::info!("cloudstorage multipart: rm_rf {}", root);
        let mut prefix_queue = vec![root];
        while !prefix_queue.is_empty() {
            let prefix = prefix_queue.pop().unwrap();
            let list_object_req = ListObjectsRequest {
                bucket: BUCKET_NAME.to_string(),
                delimiter: Some("/".to_string()),
                prefix: Some(prefix),
                ..Default::default()
            };
            let mut list_object_resp;
            while {
                list_object_resp =
                    transmute_result_for_s3error(self.client.list_objects(&list_object_req).await)?;

                if let Some(prefs) = list_object_resp.prefixes {
                    for pref in prefs {
                        prefix_queue.push(pref);
                    }
                }
                if let Some(items) = list_object_resp.items {
                    for item in items {
                        let delete_request = DeleteObjectRequest {
                            bucket: BUCKET_NAME.to_string(),
                            object: item.name,
                            ..Default::default()
                        };
                        transmute_result_for_s3error(
                            self.client.delete_object(&delete_request).await,
                        )?;
                    }
                }
                list_object_resp.next_page_token.is_some()
            } {}
        }
        Ok(())
    }

    /// returns Ok(Some(timestamp)) if the marker is in that path and everything looks good
    /// returns Ok(None) if the marker is not in that path
    /// returns Ok(None) if the marker is in that path but the timestamp is not parseable
    /// returns Err(blabla) if there was an error in accessing the marker
    async fn get_marker_contents(
        &self,
        path_root: String,
    ) -> Result<Option<DateTime<FixedOffset>>> {
        let marker_path = format!("{}/{}", path_root, "marker");
        let get_object_req = GetObjectRequest {
            bucket: BUCKET_NAME.to_string(),
            object: marker_path.clone(),
            ..Default::default()
        };
        let get_object_resp = self
            .client
            .download_object(&get_object_req, &Default::default())
            .await;
        match get_object_resp {
            Ok(body) => {
                if let Ok(ts) =
                    chrono::DateTime::parse_from_rfc3339(&String::from_utf8(body.clone())?)
                {
                    Ok(Some(ts))
                } else {
                    log::warn!("found corrupted marker file at {}. it's going to be deleted. contents were {:?}.", marker_path, body);
                    Ok(None)
                }
            }
            Err(e) => {
                if let google_cloud_storage::http::Error::Response(ErrorResponse { code, .. }) = e {
                    if code == 404 {
                        Ok(None)
                    } else {
                        Err(anyhow::anyhow!(e))
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub async fn run_cleanup_sweep(&self) -> Result<()> {
        // list all folders in the bucket. if they don't have a marker, delete them. if the marker is over EXPIRY_TIME_SECONDS, delete them.
        let list_object_req = ListObjectsRequest {
            bucket: BUCKET_NAME.to_string(),
            delimiter: Some("/".to_string()),
            ..Default::default()
        };
        let mut list_object_resp = self.client.list_objects(&list_object_req).await?;
        loop {
            // items should have nothing in it.
            if let Some(items) = list_object_resp.items {
                log::warn!("cloudstorage multipart: found {} loose items in the root outside directories. this looks like a bug. deleting them.", items.len());
                for item in items {
                    let delete_request = DeleteObjectRequest {
                        bucket: BUCKET_NAME.to_string(),
                        object: item.name,
                        ..Default::default()
                    };
                    self.client.delete_object(&delete_request).await?;
                }
            }
            if let Some(prefixes) = list_object_resp.prefixes {
                for prefix in prefixes {
                    let started = self.get_marker_contents(prefix.clone()).await;
                    match started {
                        Ok(Some(ts)) => {
                            if ts
                                < chrono::Utc::now()
                                    - chrono::Duration::seconds(
                                        EXPIRY_TIME_SECONDS.try_into().unwrap(),
                                    )
                            {
                                log::info!(
                                    "cloudstorage multipart: deleting {} because it's too old",
                                    prefix
                                );
                                self.rm_rf(prefix).await.unwrap();
                            }
                        }
                        Ok(None) => {
                            log::info!("cloudstorage multipart: deleting {} because it's missing a marker or has a malformatted marker", prefix);
                            self.rm_rf(prefix).await.unwrap();
                        }
                        Err(e) => {
                            log::error!("cloudstorage multipart: error accessing marker for {}. skipping. error was {}", prefix, e);
                        }
                    }
                }
            }
            if let Some(next_page_token) = list_object_resp.next_page_token {
                list_object_resp = self
                    .client
                    .list_objects(&ListObjectsRequest {
                        bucket: BUCKET_NAME.to_string(),
                        delimiter: Some("/".to_string()),
                        page_token: Some(next_page_token),
                        ..Default::default()
                    })
                    .await?;
            } else {
                break Ok(());
            }
        }
    }

    pub async fn upload_part(
        &self,
        client_bucket_name: SafeString,
        client_object_name: SafeString,
        upload_id: SafeString,
        part_number: u32,
        body: StreamingBlob,
    ) -> S3Result<()> {
        let part_path = multipart_loc_with_part!(
            client_bucket_name,
            client_object_name,
            upload_id,
            part_number
        );
        let up_object_req: UploadObjectRequest = UploadObjectRequest {
            bucket: BUCKET_NAME.to_string(),
            ..Default::default()
        };
        let upload_type = UploadType::Simple(Media::new(part_path));
        transmute_result_for_s3error(
            self.client
                .upload_object(
                    &up_object_req,
                    reqwest::Body::wrap_stream(body),
                    &upload_type,
                )
                .await,
        )?;
        Ok(())
    }

    pub async fn finish_upload(
        &self,
        client_bucket_name: SafeString,
        client_object_name: SafeString,
        upload_id: SafeString,
    ) -> S3Result<()> {
        let root = multipart_loc!(client_bucket_name, client_object_name, upload_id);
        // list the objects in the right folder
        let list_object_req = ListObjectsRequest {
            bucket: BUCKET_NAME.to_string(),
            delimiter: Some("/".to_string()),
            prefix: Some(root.clone()),
            ..Default::default()
        };
        let mut list_object_resp =
            transmute_result_for_s3error(self.client.list_objects(&list_object_req).await)?;
        let mut parts = PartTracker::new();
        loop {
            if let Some(items) = list_object_resp.items {
                for item in items {
                    let part_number = item.name.split("/").last().unwrap().parse::<u8>().unwrap();
                    parts.add_part(part_number);
                }
            }
            if list_object_resp.next_page_token.is_some() {
                let list_object_req = ListObjectsRequest {
                    bucket: BUCKET_NAME.to_string(),
                    page_token: list_object_resp.next_page_token,
                    ..Default::default()
                };
                list_object_resp =
                    transmute_result_for_s3error(self.client.list_objects(&list_object_req).await)?;
            } else {
                break;
            }
        }
        if !parts.is_complete() {
            return Err(s3_error!(InvalidRequest, "invalid parts"));
        }
        todo!("reconstruct");
        todo!("put into wnfs");
        // delete it from cloud storage once we're done...
        self.rm_rf(root).await?;
        Ok(())
    }
}
