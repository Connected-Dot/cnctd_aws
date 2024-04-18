use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::config::CnctdAwsConfig;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct S3File {
    // pub last_modified: String,
    // pub path: String,
    // pub size: u64,
    // pub filename: String
}

impl S3File {
    pub async fn upload(cnctd_config: CnctdAwsConfig, bucket_name: &str, filename: &str, file: &[u8], file_type: &str) -> anyhow::Result<String> {
        let region = s3::Region::from_str(&cnctd_config.region)?;
        let credentials = cnctd_config.get_s3_credentials()?;
        let bucket = s3::Bucket::new(&bucket_name, region.into(), credentials)?;
    
        let _response_data = bucket.put_object_with_content_type(&filename, file, &file_type).await?;
        let url = bucket.presign_get(&filename, 10, None)?;
        
        Ok(url)
    }

    pub async fn download(cnctd_config: CnctdAwsConfig, bucket_name: &str, path: &str) -> anyhow::Result<Vec<u8>> {
        let region = s3::Region::from_str(&cnctd_config.region)?;
        let credentials = cnctd_config.get_s3_credentials()?;
        let bucket = s3::Bucket::new(&bucket_name, region.into(), credentials)?;

        let response = bucket.get_object(path).await?;
        let bytes = response.bytes();

        Ok(bytes.to_vec())
    }

    pub async fn presigned_get_url(cnctd_config: CnctdAwsConfig, bucket_name: &str, path: &str) -> anyhow::Result<String> {
        let region = s3::Region::from_str(&cnctd_config.region)?;
        let credentials = cnctd_config.get_s3_credentials()?;
        let bucket = s3::Bucket::new(&bucket_name, region.into(), credentials)?;
        let url = bucket.presign_get(&path, 86400, None)?;
        
        Ok(url)
    }

    pub async fn presigned_put_url(cnctd_config: CnctdAwsConfig, bucket_name: &str, path: &str) -> anyhow::Result<String> {
        // let file_type: &str = mime_type.split("/").collect::<Vec<&str>>()[0];
        let region = s3::Region::from_str(&cnctd_config.region)?;
        let credentials = cnctd_config.get_s3_credentials()?;
        let bucket = s3::Bucket::new(&bucket_name, region, credentials)?;
    
        let url = bucket.presign_put(path, 20, None)?;
        
        Ok(url)
    }
    
    pub async fn delete(cnctd_config: CnctdAwsConfig, bucket_name: &str, path: &str) -> anyhow::Result<()> {
        let region = s3::Region::from_str(&cnctd_config.region)?;
        let credentials = cnctd_config.get_s3_credentials()?;
        let bucket = s3::Bucket::new(&bucket_name, region, credentials)?;
        println!("attempting to delete from {}", path);

        let _response_data = bucket.delete_object(path).await?;
    
        Ok(())
    }
}
