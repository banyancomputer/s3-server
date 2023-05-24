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
use wnfs::BlockStore;

use crate::mutex_memory_blockstore::MutexMemoryBlockStore;

pub struct WnfsS3Service {
    blockstore: dyn BlockStore + Send + Sync + 'static,
}

impl WnfsS3Service {
    pub fn new() -> Self {
        Self {
            blockstore: MutexMemoryBlockStore::new(),
        }
    }
}

#[async_trait::async_trait]
impl S3 for WnfsS3Service {
    async fn abort_multipart_upload(
        &self,
        _req: S3Request<AbortMultipartUploadInput>,
    ) -> S3Result<AbortMultipartUploadOutput> {
        Err(s3_error!(
            NotImplemented,
            "AbortMultipartUpload is not implemented yet"
        ))
    }

    async fn complete_multipart_upload(
        &self,
        _req: S3Request<CompleteMultipartUploadInput>,
    ) -> S3Result<CompleteMultipartUploadOutput> {
        Err(s3_error!(
            NotImplemented,
            "CompleteMultipartUpload is not implemented yet"
        ))
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
        _req: S3Request<CreateMultipartUploadInput>,
    ) -> S3Result<CreateMultipartUploadOutput> {
        // generate UUID

        Err(s3_error!(
            NotImplemented,
            "CreateMultipartUpload is not implemented yet"
        ))
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

    async fn upload_part(&self, _req: S3Request<UploadPartInput>) -> S3Result<UploadPartOutput> {
        // check write access to this bucket and object- which bucket and object are in the request
        todo!("check write access to this bucket and object- which bucket and object are in the request");
        // check if the upload id is valid

        // stick it in the upload part table

        // done
        
        Err(s3_error!(
            NotImplemented,
            "UploadPart is not implemented yet"
        ))
    }
}
