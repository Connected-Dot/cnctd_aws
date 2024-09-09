use std::{env, fs, str::FromStr};

use anyhow::anyhow;
use aws_types::SdkConfig;

pub struct CnctdAwsConfig {
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}

impl CnctdAwsConfig {
    pub async fn load_from_env() -> SdkConfig {
        aws_config::load_from_env().await
    }

    pub async fn from_env() -> anyhow::Result<Self> {
        let region = std::env::var("AWS_DEFAULT_REGION")?;
        let access_key = std::env::var("AWS_ACCESS_KEY_ID")?;
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")?;

        Ok(Self {
            region,
            access_key,
            secret_key,
        })
    }

    pub fn get_s3_region(config: &SdkConfig) -> anyhow::Result<s3::Region> {
        let config_region = config.region().ok_or_else(|| anyhow!("Region not found"))?.clone();
        let region = s3::Region::from_str(&config_region.to_string())?;

        Ok(region)
    }

    pub fn get_s3_credentials(&self) -> anyhow::Result<s3::creds::Credentials> {
        let credentials = s3::creds::Credentials::new(Some(&self.access_key), Some(&self.secret_key), None, None, None)?;

        Ok(credentials)
    }

}

pub struct CloudFrontConfig {
    pub key_pair_id: String,
    pub private_key: String, 
    pub resource_base: String,
    pub expiration_seconds: u64,
}

impl CloudFrontConfig {
    // Load CloudFront config from environment variables or elsewhere
    pub fn from_env() -> anyhow::Result<Self> {
        let resource_base = env::var("CLOUDFRONT_RESOURCE_URL")?;
        let key_pair_id = env::var("CLOUDFRONT_KEY_PAIR_ID")?;
        let key_path = env::var("CLOUDFRONT_PRIVATE_KEY_PATH")?;
        let private_key = fs::read_to_string(key_path)?;
        let expiration_seconds = std::env::var("CLOUDFRONT_EXPIRATION_SECONDS")?
            .parse::<u64>()?; 

        Ok(Self {
            key_pair_id,
            private_key,
            resource_base,
            expiration_seconds,
        })
    }
}