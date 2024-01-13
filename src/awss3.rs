use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
use aws_sdk_s3::primitives::ByteStream;
use std::path::Path;
use std::process;
use std::sync::Arc;
use clap::Parser;

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

pub fn 
upload_object(
    data: &str,
    client: Client,
    //bucket: &str,
) -> Result<(), Error> {
    // this needs to append to the existing log data object, not overwrite it.
    //let body = ByteStream::from_static(data.as_bytes());
    println!("data: {data:?}");
    let resp = &client
        .put_object()
        .bucket("logcollectionbucket")
        .key(data)
        //.body(body)
        .send();

    //println!("Upload success. Version: {:?}", resp.version_id);

    //let resp = client.get_object()
    //            .bucket("logcollectionbucket").key(data).send().await?;
    //let d = resp.body.collect().await;
    //println!("data: {:?}", d.unwrap().into_bytes());
    Ok(())
}


pub async fn 
start_s3() -> Result<Client, Error> {
    //https://docs.rs/aws-config/latest/aws_config/index.html
    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config); 

    //// Test upload event object
    //let bucket = String::from("logcollectionbucket");
    //let data = String::from("another test before stopping");
    //let result = upload_object(&client,&bucket,&data).await;
    //match result {
    //    Ok(val) => println!("{val:?}"),
    //    Err(err) => eprintln!("{err:?}"),
    //}

    Ok(client)
}
