use aws_config::meta::region::RegionProviderChain;
use aws_sdk_kms::{Client, Error};

pub async fn get_kms_client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    Ok(client)
}
