use s3s::S3;

pub struct WnfsS3Service {}

impl WnfsS3Service {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl S3 for WnfsS3Service {
    // big ol todo... put things in here as you implement them
}
