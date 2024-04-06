use aws_types::SdkConfig;

pub struct CnctdAwsConfig;

impl CnctdAwsConfig {
    pub async fn load_from_env() -> SdkConfig {
        aws_config::load_from_env().await
    }
}