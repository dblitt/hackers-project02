use aws_sdk_dynamodb::{Client, Error};
use aws_config::{meta::region::RegionProviderChain, Region};

pub async fn get_dynamodb_client() -> Result<Client, Error> {
    // let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // let config = aws_config::from_env().region(region_provider).load().await;
    let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
    let client = Client::new(&config);
    Ok(client)
}