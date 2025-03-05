use {
    crate::{
        errors::s3::S3Error,
        utils::objects::types::{Object, ObjectStore},
    },
    aws_config::{retry::RetryConfig, BehaviorVersion, Region, SdkConfig},
    aws_credential_types::provider::SharedCredentialsProvider,
    aws_sdk_s3::{
        operation::put_object::PutObjectOutput, primitives::ByteStream, Client,
        Config,
    },
    uuid::Uuid,
};

pub struct S3 {
    client: Client,
}

impl S3 {
    pub fn new(
        url: &str,
        access_key_id: &str,
        secret_access_key: &str,
        provider_name: &str,
    ) -> Self {
        let pre_credentials = aws_credential_types::Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            provider_name,
        );

        let bbb = SharedCredentialsProvider::new(pre_credentials);

        let shared_config = SdkConfig::builder()
            .behavior_version(BehaviorVersion::latest())
            .credentials_provider(bbb)
            .endpoint_url(url)
            .region(Region::new("us-east-1"))
            .retry_config(RetryConfig::standard().with_max_attempts(5))
            .build();

        let s3_config = Config::from(&shared_config);

        let client = Client::from_conf(s3_config);

        Self { client }
    }
}

impl ObjectStore for S3 {
    async fn upload(&self, obj: Object) -> Result<PutObjectOutput, S3Error> {
        let result = self
            .client
            .put_object()
            .bucket(obj.bucket)
            .body(ByteStream::from(obj.content))
            .content_type(obj.content_type.to_string())
            .send()
            .await
            .map_err(|_err| S3Error::Generic)?;

        Ok(result)
    }

    async fn get(&self, key: Uuid, bucket: String) -> Result<(), S3Error> {
        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|_err| S3Error::Generic)?;

        let body =
            resp.body.collect().await.map_err(|_err| S3Error::Generic)?;

        let _bytes = body.into_bytes(); // todo: update later

        Ok(())
    }
}
