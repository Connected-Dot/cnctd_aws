use aws_sdk_secretsmanager::Client;
use anyhow::anyhow;
use crate::config::CnctdAwsConfig;

pub struct CnctdAwsSecrets;

impl CnctdAwsSecrets {
    pub async fn create_secret(
        cnctd_config: CnctdAwsConfig,
        key: &str,
        value: &str,
    ) -> anyhow::Result<()> {
        let sdk_config = cnctd_config.get_aws_sdk_config().await?;
        println!(
            "Creating secret: {} with value: {}, config: {:?}",
            key,
            value,
            sdk_config.endpoint_url()
        );
        let client = Client::new(&sdk_config);
        let _response = client
            .create_secret()
            .name(key)
            .secret_string(value)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_secret(
        cnctd_config: CnctdAwsConfig,
        key: &str,
    ) -> anyhow::Result<String> {
        let sdk_config = cnctd_config.get_aws_sdk_config().await?;
        let client = Client::new(&sdk_config);
        let response = client
            .get_secret_value()
            .secret_id(key)
            .send()
            .await?;

        let secret = response.secret_string.ok_or(anyhow!("Secret not found"))?;
        Ok(secret)
    }

    pub async fn delete_secret(
        cnctd_config: CnctdAwsConfig,
        key: &str,
    ) -> anyhow::Result<()> {
        let sdk_config = cnctd_config.get_aws_sdk_config().await?;
        let client = Client::new(&sdk_config);
        let _response = client
            .delete_secret()
            .secret_id(key)
            .send()
            .await?;

        Ok(())
    }

    pub async fn update_secret(
        cnctd_config: CnctdAwsConfig,
        key: &str,
        value: &str,
    ) -> anyhow::Result<()> {
        let sdk_config = cnctd_config.get_aws_sdk_config().await?;
        let client = Client::new(&sdk_config);
        let _response = client
            .update_secret()
            .secret_id(key)
            .secret_string(value)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_secrets(
        cnctd_config: CnctdAwsConfig,
    ) -> anyhow::Result<Vec<String>> {
        let sdk_config = cnctd_config.get_aws_sdk_config().await?;
        let client = Client::new(&sdk_config);
        let response = client.list_secrets().send().await?;

        let secret_ids = response
            .secret_list
            .ok_or(anyhow!("Secrets not found"))?
            .iter()
            .filter_map(|secret| secret.name.clone())
            .collect();

        Ok(secret_ids)
    }

    pub async fn permanently_delete_secret(
        cnctd_config: CnctdAwsConfig,
        key: &str,
    ) -> anyhow::Result<()> {
        let sdk_config = cnctd_config.get_aws_sdk_config().await?;
        let client = Client::new(&sdk_config);
        let _response = client
            .delete_secret()
            .secret_id(key)
            .force_delete_without_recovery(true)
            .send()
            .await?;

        Ok(())
    }
}