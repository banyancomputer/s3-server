use s3s::{auth::{SecretKey, S3Auth}, S3Result};

pub struct BanyanS3Auth {}

impl BanyanS3Auth {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl S3Auth for BanyanS3Auth {
    async fn get_secret_key(&self, _access_key: &str) -> S3Result<SecretKey> {
        // match self.lookup(access_key) {
        //     None => Err(s3_error!(NotSignedUp, "Your account is not signed up")),
        //     Some(s) => Ok(s.clone()),
        // }
        todo!("BanyanS3Auth::get_secret_key")
    }
}
