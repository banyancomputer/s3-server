use google_cloud_default::WithAuthExt;
use google_cloud_storage::{client::{ClientConfig, Client}};

struct CloudStorageForMultipartConstruction {
    client: Client,
}

impl CloudStorageForMultipartConstruction {
    pub async fn new() -> Self {
        let config = ClientConfig::default().with_auth().await.unwrap();
        let client = Client::new(config);

        Self { client }
    }

    pub fn check_part_id(&self, bucket_name: String, object_name: String, upload_id: uuid::v4, part_id: u32) {
        // check that the part_id exists in the cloud storage bucket
        self.client.get_object(bucket_name, object_name).await.unwrap();
    }
}
