use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client, Error};
use aws_sdk_s3::primitives::ByteStream;
//use s3_service::error::Error;

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

pub async fn 
upload_object(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<(), Error> {

    // this needs to append to the existing log data object, not overwrite it.
    let body = ByteStream::from_static("test the content".as_bytes());
    println!("body: {body:?}, bucket: {bucket}, key: {key}");
    let resp = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    println!("Upload success. Version: {:?}", resp.version_id);
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    let data = resp.body.collect().await;
    println!("data: {:?}", data.unwrap().into_bytes());
    Ok(())
}


// Client::new is an expesive operation. Decompose this function as well as 
// upload_object so that it takes the log data in the producer function.
//#[tokio::main]
pub async fn 
start_s3() -> Result<Client, Error> {
//start_s3() -> Result<(), Error> {
    //https://docs.rs/aws-config/latest/aws_config/index.html
    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let tc = Client::new(&shared_config); 
    // Test upload event object
    let bucket = String::from("endepointe");
    let key = String::from("ruletest.txt");
    let result = upload_object(&tc,&bucket,&key).await;
    match result {
        Ok(val) => println!("{val:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
    ///////////////////////////////////////////////////

    //Ok(Client::new(&shared_config))
    Ok(tc)
}
