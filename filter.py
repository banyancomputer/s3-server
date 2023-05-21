import re

data = '''
    async fn abort_multipart_upload(&self, _req: S3Request<AbortMultipartUploadInput>) -> S3Result<AbortMultipartUploadOutput> {
    async fn complete_multipart_upload(
    async fn copy_object(&self, _req: S3Request<CopyObjectInput>) -> S3Result<CopyObjectOutput> {
    async fn create_bucket(&self, _req: S3Request<CreateBucketInput>) -> S3Result<CreateBucketOutput> {
    async fn create_multipart_upload(
    async fn delete_bucket(&self, _req: S3Request<DeleteBucketInput>) -> S3Result<DeleteBucketOutput> {
    async fn delete_bucket_analytics_configuration(
    async fn delete_bucket_cors(&self, _req: S3Request<DeleteBucketCorsInput>) -> S3Result<DeleteBucketCorsOutput> {
    async fn delete_bucket_encryption(
    async fn delete_bucket_intelligent_tiering_configuration(
    async fn delete_bucket_inventory_configuration(
    async fn delete_bucket_lifecycle(
    async fn delete_bucket_metrics_configuration(
    async fn delete_bucket_ownership_controls(
    async fn delete_bucket_policy(&self, _req: S3Request<DeleteBucketPolicyInput>) -> S3Result<DeleteBucketPolicyOutput> {
    async fn delete_bucket_replication(
    async fn delete_bucket_tagging(&self, _req: S3Request<DeleteBucketTaggingInput>) -> S3Result<DeleteBucketTaggingOutput> {
    async fn delete_bucket_website(&self, _req: S3Request<DeleteBucketWebsiteInput>) -> S3Result<DeleteBucketWebsiteOutput> {
    async fn delete_object(&self, _req: S3Request<DeleteObjectInput>) -> S3Result<DeleteObjectOutput> {
    async fn delete_object_tagging(&self, _req: S3Request<DeleteObjectTaggingInput>) -> S3Result<DeleteObjectTaggingOutput> {
    async fn delete_objects(&self, _req: S3Request<DeleteObjectsInput>) -> S3Result<DeleteObjectsOutput> {
    async fn delete_public_access_block(
    async fn get_bucket_accelerate_configuration(
    async fn get_bucket_acl(&self, _req: S3Request<GetBucketAclInput>) -> S3Result<GetBucketAclOutput> {
    async fn get_bucket_analytics_configuration(
    async fn get_bucket_cors(&self, _req: S3Request<GetBucketCorsInput>) -> S3Result<GetBucketCorsOutput> {
    async fn get_bucket_encryption(&self, _req: S3Request<GetBucketEncryptionInput>) -> S3Result<GetBucketEncryptionOutput> {
    async fn get_bucket_intelligent_tiering_configuration(
    async fn get_bucket_inventory_configuration(
    async fn get_bucket_lifecycle_configuration(
    async fn get_bucket_location(&self, _req: S3Request<GetBucketLocationInput>) -> S3Result<GetBucketLocationOutput> {
    async fn get_bucket_logging(&self, _req: S3Request<GetBucketLoggingInput>) -> S3Result<GetBucketLoggingOutput> {
    async fn get_bucket_metrics_configuration(
    async fn get_bucket_notification_configuration(
    async fn get_bucket_ownership_controls(
    async fn get_bucket_policy(&self, _req: S3Request<GetBucketPolicyInput>) -> S3Result<GetBucketPolicyOutput> {
    async fn get_bucket_policy_status(
    async fn get_bucket_replication(&self, _req: S3Request<GetBucketReplicationInput>) -> S3Result<GetBucketReplicationOutput> {
    async fn get_bucket_request_payment(
    async fn get_bucket_tagging(&self, _req: S3Request<GetBucketTaggingInput>) -> S3Result<GetBucketTaggingOutput> {
    async fn get_bucket_versioning(&self, _req: S3Request<GetBucketVersioningInput>) -> S3Result<GetBucketVersioningOutput> {
    async fn get_bucket_website(&self, _req: S3Request<GetBucketWebsiteInput>) -> S3Result<GetBucketWebsiteOutput> {
    async fn get_object(&self, _req: S3Request<GetObjectInput>) -> S3Result<GetObjectOutput> {
    async fn get_object_acl(&self, _req: S3Request<GetObjectAclInput>) -> S3Result<GetObjectAclOutput> {
    async fn get_object_attributes(&self, _req: S3Request<GetObjectAttributesInput>) -> S3Result<GetObjectAttributesOutput> {
    async fn get_object_legal_hold(&self, _req: S3Request<GetObjectLegalHoldInput>) -> S3Result<GetObjectLegalHoldOutput> {
    async fn get_object_lock_configuration(
    async fn get_object_retention(&self, _req: S3Request<GetObjectRetentionInput>) -> S3Result<GetObjectRetentionOutput> {
    async fn get_object_tagging(&self, _req: S3Request<GetObjectTaggingInput>) -> S3Result<GetObjectTaggingOutput> {
    async fn get_object_torrent(&self, _req: S3Request<GetObjectTorrentInput>) -> S3Result<GetObjectTorrentOutput> {
    async fn get_public_access_block(&self, _req: S3Request<GetPublicAccessBlockInput>) -> S3Result<GetPublicAccessBlockOutput> {
    async fn head_bucket(&self, _req: S3Request<HeadBucketInput>) -> S3Result<HeadBucketOutput> {
    async fn head_object(&self, _req: S3Request<HeadObjectInput>) -> S3Result<HeadObjectOutput> {
    async fn list_bucket_analytics_configurations(
    async fn list_bucket_intelligent_tiering_configurations(
    async fn list_bucket_inventory_configurations(
    async fn list_bucket_metrics_configurations(
    async fn list_buckets(&self, _req: S3Request<ListBucketsInput>) -> S3Result<ListBucketsOutput> {
    async fn list_multipart_uploads(&self, _req: S3Request<ListMultipartUploadsInput>) -> S3Result<ListMultipartUploadsOutput> {
    async fn list_object_versions(&self, _req: S3Request<ListObjectVersionsInput>) -> S3Result<ListObjectVersionsOutput> {
    async fn list_objects(&self, _req: S3Request<ListObjectsInput>) -> S3Result<ListObjectsOutput> {
    async fn list_objects_v2(&self, _req: S3Request<ListObjectsV2Input>) -> S3Result<ListObjectsV2Output> {
    async fn list_parts(&self, _req: S3Request<ListPartsInput>) -> S3Result<ListPartsOutput> {
    async fn put_bucket_accelerate_configuration(
    async fn put_bucket_acl(&self, _req: S3Request<PutBucketAclInput>) -> S3Result<PutBucketAclOutput> {
    async fn put_bucket_analytics_configuration(
    async fn put_bucket_cors(&self, _req: S3Request<PutBucketCorsInput>) -> S3Result<PutBucketCorsOutput> {
    async fn put_bucket_encryption(&self, _req: S3Request<PutBucketEncryptionInput>) -> S3Result<PutBucketEncryptionOutput> {
    async fn put_bucket_intelligent_tiering_configuration(
    async fn put_bucket_inventory_configuration(
    async fn put_bucket_lifecycle_configuration(
    async fn put_bucket_logging(&self, _req: S3Request<PutBucketLoggingInput>) -> S3Result<PutBucketLoggingOutput> {
    async fn put_bucket_metrics_configuration(
    async fn put_bucket_notification_configuration(
    async fn put_bucket_ownership_controls(
    async fn put_bucket_policy(&self, _req: S3Request<PutBucketPolicyInput>) -> S3Result<PutBucketPolicyOutput> {
    async fn put_bucket_replication(&self, _req: S3Request<PutBucketReplicationInput>) -> S3Result<PutBucketReplicationOutput> {
    async fn put_bucket_request_payment(
    async fn put_bucket_tagging(&self, _req: S3Request<PutBucketTaggingInput>) -> S3Result<PutBucketTaggingOutput> {
    async fn put_bucket_versioning(&self, _req: S3Request<PutBucketVersioningInput>) -> S3Result<PutBucketVersioningOutput> {
    async fn put_bucket_website(&self, _req: S3Request<PutBucketWebsiteInput>) -> S3Result<PutBucketWebsiteOutput> {
    async fn put_object(&self, _req: S3Request<PutObjectInput>) -> S3Result<PutObjectOutput> {
    async fn put_object_acl(&self, _req: S3Request<PutObjectAclInput>) -> S3Result<PutObjectAclOutput> {
    async fn put_object_legal_hold(&self, _req: S3Request<PutObjectLegalHoldInput>) -> S3Result<PutObjectLegalHoldOutput> {
    async fn put_object_lock_configuration(
    async fn put_object_retention(&self, _req: S3Request<PutObjectRetentionInput>) -> S3Result<PutObjectRetentionOutput> {
    async fn put_object_tagging(&self, _req: S3Request<PutObjectTaggingInput>) -> S3Result<PutObjectTaggingOutput> {
    async fn put_public_access_block(&self, _req: S3Request<PutPublicAccessBlockInput>) -> S3Result<PutPublicAccessBlockOutput> {
    async fn restore_object(&self, _req: S3Request<RestoreObjectInput>) -> S3Result<RestoreObjectOutput> {
    async fn select_object_content(&self, _req: S3Request<SelectObjectContentInput>) -> S3Result<SelectObjectContentOutput> {
    async fn upload_part(&self, _req: S3Request<UploadPartInput>) -> S3Result<UploadPartOutput> {
    async fn upload_part_copy(&self, _req: S3Request<UploadPartCopyInput>) -> S3Result<UploadPartCopyOutput> {
    async fn write_get_object_response(
'''

matches = re.findall(r'async fn (\w+)\(', data)
for match in matches:
    print(match)
