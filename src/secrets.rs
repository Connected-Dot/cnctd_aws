use aws_sdk_secretsmanager::Client;
use aws_types::SdkConfig;
use anyhow::anyhow;

pub struct CnctdAwsSecrets;


impl CnctdAwsSecrets {
    pub async fn create_secret(config: SdkConfig, key: &str, value: &str) -> anyhow::Result<()> {
        println!("Creating secret: {} with value: {}, config: {:?}", key, value, config.endpoint_url());
        let client = Client::new(&config);
        let _response = client.create_secret()
            .name(key)
            .secret_string(value)
            .send()
            .await?;

       Ok(())
   }

   pub async fn get_secret(config: SdkConfig, key: &str) -> anyhow::Result<String> {
        let client = Client::new(&config);
        let response = client.get_secret_value()
            .secret_id(key)
            .send()
            .await?;

        let secret = response.secret_string.ok_or(anyhow!("Secret not found"))?;
        Ok(secret)
    }

    pub async fn delete_secret(config: SdkConfig, key: &str) -> anyhow::Result<()> {
        let client = Client::new(&config);
        let _response = client.delete_secret()
            .secret_id(key)
            .send()
            .await?;

        Ok(())
    }

    pub async fn update_secret(config: SdkConfig, key: &str, value: &str) -> anyhow::Result<()> {
        let client = Client::new(&config);
        let _response = client.update_secret()
            .secret_id(key)
            .secret_string(value)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_secrets(config: SdkConfig) -> anyhow::Result<Vec<String>> {
        let client = Client::new(&config);
        let response = client.list_secrets()
            .send()
            .await?;

        let secret_ids = response.secret_list
            .ok_or(anyhow!("Secrets not found"))?
            .iter()
            .filter_map(|secret| secret.name.clone())
            .collect();

        Ok(secret_ids)
    }

    pub async fn permanently_delete_secret(config: SdkConfig, key: &str) -> anyhow::Result<()> {
        let client = Client::new(&config);
        let _response = client.delete_secret()
            .secret_id(key)
            .force_delete_without_recovery(true)
            .send()
            .await?;

        Ok(())
    }

}