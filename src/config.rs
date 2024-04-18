use std::str::FromStr;

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