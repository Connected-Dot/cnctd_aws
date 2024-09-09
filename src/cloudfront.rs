use std::time::{SystemTime, UNIX_EPOCH};

use cloudfront_sign::{get_signed_url, SignedOptions};
use urlencoding::encode;

use crate::config::CloudFrontConfig;

pub struct CloudFront;

impl CloudFront {
    pub fn get_presigned_url(config: &CloudFrontConfig, path: &str) -> anyhow::Result<String> {
        
        let options = SignedOptions {
            key_pair_id: config.key_pair_id.clone(),
            private_key: config.private_key.clone(),
            date_less_than: (SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs() + config.expiration_seconds),
            ..Default::default()  // Default values for optional fields
        };
    
        // URL encode the file path (this handles spaces and special characters)
        let encoded_path = encode(&path); // Encode the path from the request message
    
        // Construct the full resource URL (CloudFront base URL + encoded path)
        let file_path = format!("{}/{}", config.resource_base, encoded_path);
    
        // Generate the signed URL
        let url = get_signed_url(&file_path, &options)?;
    
        Ok(url)
    }
}