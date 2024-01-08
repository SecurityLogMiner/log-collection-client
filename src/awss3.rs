use aws_config::meta::region::RegionProviderChain;

use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client};

use aws_sdk_s3::primitives::ByteStream;
use s3_service::error::Error;

/*
use clap::Parser;
#[derive(Debug, Parser)]
pub struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// The name of the log data object in the bucket.
    #[structopt(short, long)]
    key: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}
*/

pub fn 
upload_object(
    data: &str,
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<(), Error> {
    // this needs to append to the existing log data object, not overwrite it.
    let body = ByteStream::from_static("hello world this is a test".as_bytes());
    let _resp = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send();
        //.await?; // is this needed?
    Ok(())
}


// Client::new is an expesive operation. Decompose this function as well as 
// upload_object so that it takes the log data in the producer function.
#[tokio::main]
pub async fn 
start_s3() -> Result<Client, Error> {

    //https://docs.rs/aws-config/latest/aws_config/index.html
    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("us-west-2"));

    println!();
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    Ok(Client::new(&shared_config))
}
