use std::{sync::Arc};

use s3s::{
    dto::{
        AbortMultipartUploadInput, AbortMultipartUploadOutput, CompleteMultipartUploadInput,
        CompleteMultipartUploadOutput, CopyObjectInput, CopyObjectOutput, CreateBucketInput,
        CreateBucketOutput, CreateMultipartUploadInput, CreateMultipartUploadOutput,
        DeleteBucketCorsInput, DeleteBucketCorsOutput, DeleteBucketInput, DeleteBucketOutput,
        DeleteObjectInput, DeleteObjectOutput, GetBucketAclInput, GetBucketAclOutput,
        GetBucketCorsInput, GetBucketCorsOutput, GetBucketLifecycleConfigurationInput,
        GetBucketLifecycleConfigurationOutput, GetBucketLocationInput, GetBucketLocationOutput,
        GetBucketLoggingInput, GetBucketLoggingOutput, GetBucketVersioningInput,
        GetBucketVersioningOutput, GetObjectAclInput, GetObjectAclOutput, GetObjectInput,
        GetObjectOutput, HeadBucketInput, HeadBucketOutput, HeadObjectInput, HeadObjectOutput,
        ListBucketsInput, ListBucketsOutput, ListObjectsInput, ListObjectsOutput,
        ListObjectsV2Input, ListObjectsV2Output, PutBucketAclInput, PutBucketAclOutput,
        PutBucketCorsInput, PutBucketCorsOutput, PutObjectAclInput, PutObjectAclOutput,
        PutObjectInput, PutObjectOutput, UploadPartInput, UploadPartOutput,
    },
    s3_error, S3Request, S3Result, S3,
};

use crate::{multipart_uploads::CloudStorageForMultipartConstruction, banyan_s3_auth::BanyanS3Auth};

pub struct WnfsS3Service {
    //blockstore: MutexMemoryBlockStore,
    multipart_cloud_storage: CloudStorageForMultipartConstruction,
    auth: Arc<BanyanS3Auth>,
}

impl WnfsS3Service {
    pub async fn new(auth: Arc<BanyanS3Auth>) -> Self {
        Self {
            //blockstore: MutexMemoryBlockStore::new(),
            multipart_cloud_storage: CloudStorageForMultipartConstruction::new().await,
            auth: auth.clone(),
        }
    }
}

#[async_trait::async_trait]
impl S3 for WnfsS3Service {
    async fn abort_multipart_upload(
        &self,
        req: S3Request<AbortMultipartUploadInput>,
    ) -> S3Result<AbortMultipartUploadOutput> {
        if !self.auth.as_ref().has_write_permission_to_bucket(
            req.credentials,
            req.input.bucket,
        )? {
            return Err(s3_error!(
                AccessDenied,
                "You do not have write permission to this bucket"
            ));
        };

        self.multipart_cloud_storage
            .cleanup_upload(
                req.input.bucket.into(),
                req.input.key.into(),
                req.input.upload_id.into(),
            )
            .await?;
        Ok(AbortMultipartUploadOutput {
            ..Default::default()
        })
    }

    async fn complete_multipart_upload(
        &self,
        req: S3Request<CompleteMultipartUploadInput>,
    ) -> S3Result<CompleteMultipartUploadOutput> {
        if !self.auth.as_ref().has_write_permission_to_bucket(
            req.credentials,
            req.input.bucket,
        )? {
            return Err(s3_error!(
                AccessDenied,
                "You do not have write permission to this bucket"
            ));
        };
        self.multipart_cloud_storage
            .finish_upload(
                req.input.bucket.into(),
                req.input.key.into(),
                req.input.upload_id.into(),
            )
            .await?;
        Ok(CompleteMultipartUploadOutput {
            ..Default::default()
        })
    }

    async fn copy_object(&self, _req: S3Request<CopyObjectInput>) -> S3Result<CopyObjectOutput> {
        Err(s3_error!(
            NotImplemented,
            "CopyObject is not implemented yet"
        ))
    }

    async fn create_bucket(
        &self,
        _req: S3Request<CreateBucketInput>,
    ) -> S3Result<CreateBucketOutput> {
        Err(s3_error!(
            NotImplemented,
            "CreateBucket is not implemented yet"
        ))
    }

    async fn create_multipart_upload(
        &self,
        req: S3Request<CreateMultipartUploadInput>,
    ) -> S3Result<CreateMultipartUploadOutput> {
        if !self.auth.as_ref().has_write_permission_to_bucket(
            req.credentials,
            req.input.bucket,
        )? {
            return Err(s3_error!(
                AccessDenied,
                "You do not have write permission to this bucket"
            ));
        };
        // generate UUID
        let uuid = uuid::Uuid::new_v4().to_string();
        // create multipart upload
        self.multipart_cloud_storage
            .create_multipart_upload_folder(
                req.input.bucket.into(),
                req.input.key.into(),
                uuid.into(),
            )
            .await?;
        // return that uuid
        Ok(CreateMultipartUploadOutput {
            bucket: Some(req.input.bucket),
            key: Some(req.input.key),
            upload_id: Some(uuid.to_string()),
            ..Default::default()
        })
    }

    async fn delete_bucket(
        &self,
        _req: S3Request<DeleteBucketInput>,
    ) -> S3Result<DeleteBucketOutput> {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucket is not implemented yet"
        ))
    }

    async fn delete_bucket_cors(
        &self,
        _req: S3Request<DeleteBucketCorsInput>,
    ) -> S3Result<DeleteBucketCorsOutput> {
        Err(s3_error!(
            NotImplemented,
            "DeleteBucketCors is not implemented yet"
        ))
    }

    async fn delete_object(
        &self,
        _req: S3Request<DeleteObjectInput>,
    ) -> S3Result<DeleteObjectOutput> {
        Err(s3_error!(
            NotImplemented,
            "DeleteObject is not implemented yet"
        ))
    }

    async fn get_bucket_acl(
        &self,
        _req: S3Request<GetBucketAclInput>,
    ) -> S3Result<GetBucketAclOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketAcl is not implemented yet"
        ))
    }

    async fn get_bucket_cors(
        &self,
        _req: S3Request<GetBucketCorsInput>,
    ) -> S3Result<GetBucketCorsOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketCors is not implemented yet"
        ))
    }

    async fn get_bucket_lifecycle_configuration(
        &self,
        _req: S3Request<GetBucketLifecycleConfigurationInput>,
    ) -> S3Result<GetBucketLifecycleConfigurationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketLifecycleConfiguration is not implemented yet"
        ))
    }

    async fn get_bucket_location(
        &self,
        _req: S3Request<GetBucketLocationInput>,
    ) -> S3Result<GetBucketLocationOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketLocation is not implemented yet"
        ))
    }

    async fn get_bucket_logging(
        &self,
        _req: S3Request<GetBucketLoggingInput>,
    ) -> S3Result<GetBucketLoggingOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketLogging is not implemented yet"
        ))
    }

    async fn list_buckets(&self, _req: S3Request<ListBucketsInput>) -> S3Result<ListBucketsOutput> {
        Err(s3_error!(
            NotImplemented,
            "ListBuckets is not implemented yet"
        ))
    }

    async fn list_objects(&self, _req: S3Request<ListObjectsInput>) -> S3Result<ListObjectsOutput> {
        Err(s3_error!(
            NotImplemented,
            "ListObjects is not implemented yet"
        ))
    }

    async fn get_bucket_versioning(
        &self,
        _req: S3Request<GetBucketVersioningInput>,
    ) -> S3Result<GetBucketVersioningOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetBucketVersioning is not implemented yet"
        ))
    }

    async fn get_object(&self, _req: S3Request<GetObjectInput>) -> S3Result<GetObjectOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetObject is not implemented yet"
        ))
    }

    async fn get_object_acl(
        &self,
        _req: S3Request<GetObjectAclInput>,
    ) -> S3Result<GetObjectAclOutput> {
        Err(s3_error!(
            NotImplemented,
            "GetObjectAcl is not implemented yet"
        ))
    }

    async fn list_objects_v2(
        &self,
        _req: S3Request<ListObjectsV2Input>,
    ) -> S3Result<ListObjectsV2Output> {
        Err(s3_error!(
            NotImplemented,
            "ListObjectsV2 is not implemented yet"
        ))
    }

    async fn head_bucket(&self, _req: S3Request<HeadBucketInput>) -> S3Result<HeadBucketOutput> {
        Err(s3_error!(
            NotImplemented,
            "HeadBucket is not implemented yet"
        ))
    }

    async fn put_bucket_acl(
        &self,
        _req: S3Request<PutBucketAclInput>,
    ) -> S3Result<PutBucketAclOutput> {
        Err(s3_error!(
            NotImplemented,
            "PutBucketAcl is not implemented yet"
        ))
    }

    async fn head_object(&self, _req: S3Request<HeadObjectInput>) -> S3Result<HeadObjectOutput> {
        Err(s3_error!(
            NotImplemented,
            "HeadObject is not implemented yet"
        ))
    }

    async fn put_bucket_cors(
        &self,
        _req: S3Request<PutBucketCorsInput>,
    ) -> S3Result<PutBucketCorsOutput> {
        Err(s3_error!(
            NotImplemented,
            "PutBucketCors is not implemented yet"
        ))
    }

    async fn put_object(&self, _req: S3Request<PutObjectInput>) -> S3Result<PutObjectOutput> {
        Err(s3_error!(
            NotImplemented,
            "PutObject is not implemented yet"
        ))
    }

    async fn put_object_acl(
        &self,
        _req: S3Request<PutObjectAclInput>,
    ) -> S3Result<PutObjectAclOutput> {
        Err(s3_error!(
            NotImplemented,
            "PutObjectAcl is not implemented yet"
        ))
    }

    async fn upload_part(&self, req: S3Request<UploadPartInput>) -> S3Result<UploadPartOutput> {
        // check write access 
        if !self.auth.as_ref().has_write_permission_to_bucket(
            req.credentials,
            req.input.bucket,
        )? {
            return Err(s3_error!(
                AccessDenied,
                "You do not have write permission to this bucket"
            ));
        };
        // check if the upload id is valid
        if !self
            .multipart_cloud_storage
            .check_upload_exists(
                req.input.bucket.into(),
                req.input.key.into(),
                req.input.upload_id.into(),
            )
            .await?
        {
            return Err(s3_error!(
                NoSuchUpload,
                "The specified multipart upload does not exist. The upload ID might be invalid, or the multipart upload might have been aborted or completed."
            ));
        }
        if req.input.body.is_none() {
            return Err(s3_error!(NotImplemented, "UploadPart without a body???"));
        }
        // stick it in the upload part table
        self.multipart_cloud_storage
            .upload_part(
                req.input.bucket.into(),
                req.input.key.into(),
                req.input.upload_id.into(),
                req.input.part_number as u32,
                req.input.body.unwrap(),
            )
            .await?;
        // done
        Ok(UploadPartOutput {
            ..Default::default()
        })
    }
}
