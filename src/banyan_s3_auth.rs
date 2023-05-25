use anyhow::Result;
use std::{sync::Arc};

use firestore::FirestoreDb;
use s3s::{
    auth::{S3Auth, SecretKey, Credentials},
    s3_error, S3Result,
};
use serde::Deserialize;

pub struct BanyanS3Auth {
    auth_database_connection: Arc<FirestoreDb>,
    key_database_connection: Arc<FirestoreDb>,
}

#[derive(Debug, Deserialize)]
// TODO get this right... this should be what's in the firestore db
pub struct BanyanUser {
    pub id: String,
    pub is_s3_enabled: bool,
    pub metadata: String,
}

struct SKWrap(SecretKey);

impl<'de> Deserialize<'de> for SKWrap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(SKWrap(SecretKey::from(s)))
    }
}

// TODO maybe authentication and authorization should be separate functions?
// TODO should they all be in the same database? I'm not convinced of anything at this point,
// TODO vera thinks maybe same data base and we just write a super duper aggro test suite... maybe a good idea
// i have anxiety
impl BanyanS3Auth {
    pub async fn new(auth_endpoint: String, key_endpoint: String) -> Result<Self> {
        let auth_database_connection = Arc::new(FirestoreDb::new(auth_endpoint).await?);
        let key_database_connection = Arc::new(FirestoreDb::new(key_endpoint).await?);
        Ok(Self {
            auth_database_connection,
            key_database_connection,
        })
    }

    // you get notsignedup, custom, accessdenied, InvalidAccessKeyId, InternalError
    /// Authenticate that the access key is valid and allowed to be used for s3 stuff
    pub async fn authenticate_and_check_s3_permissions(&self, access_key: &str) -> S3Result<()> {
        let user: BanyanUser = self
            .auth_database_connection
            .fluent()
            .select()
            .by_id_in("ACCESS_KEYS")
            .obj()
            .one(access_key)
            .await
            // this code couldn't connect to auth database/couldn't query
            .map_err(|e| {
                s3_error!(
                    InternalError,
                    "Error looking up access key in auth database: {}",
                    e
                )
            })?
            // access key wasn't there
            .ok_or(s3_error!(
                InvalidAccessKeyId,
                "Access key not found in auth database"
            ))?;
        // check if user is allowed to use s3
        if !user.is_s3_enabled {
            return Err(s3_error!(
                NotSignedUp,
                "Your account is not signed up for s3"
            ));
        }
        // all good!
        Ok(())
    }

    // TODO could this be "outsourced to security rules"? - vera
    pub async fn get_decryption_key_from_db(&self, access_key: &str) -> S3Result<SecretKey> {
        let skw: SKWrap = self
            .key_database_connection
            .fluent()
            .select()
            .by_id_in("KEYS")
            .obj()
            .one(access_key)
            .await
            .map_err(|e| {
                s3_error!(
                    InternalError,
                    "Error looking up decryption key in key database: {}",
                    e
                )
            })?
            .ok_or(s3_error!(
                InvalidAccessKeyId,
                "decryption key not found in key database"
            ))?;
        Ok(skw.0)
    }

    pub fn has_write_permission_to_bucket(&self, credentials: Option<Credentials>, bucket_name: String) -> S3Result<bool> {
        unimplemented!("see how you're passing an 'auth' into wnfss3service? is that good...? is there a better way to do this?");
    }
}

#[async_trait::async_trait]
impl S3Auth for BanyanS3Auth {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        // first, authenticate that the auth database says that the access key is valid and allowed to be used for s3 stuff
        self.authenticate_and_check_s3_permissions(access_key)
            .await?;
        // then, if it is, look up the secret key in the key database
        self.get_decryption_key_from_db(access_key).await
    }
}

#[async_trait::async_trait]
impl S3Auth for &'static BanyanS3Auth {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        self.get_secret_key(access_key).await
    }
}