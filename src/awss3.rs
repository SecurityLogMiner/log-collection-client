use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
use aws_sdk_s3::primitives::ByteStream;
use std::path::Path;
use std::process;
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

pub async fn 
upload_object(
    client: &Client,
    bucket: &str,
    data: &str,
) -> Result<(), Error> {
    // this needs to append to the existing log data object, not overwrite it.
    //let body = ByteStream::from_static(data.as_bytes());
    //println!("body: {body:?}");
    let resp = client
        .put_object()
        .bucket("logcollectionbucket")
        .key(data)
        //.body(body)
        .send()
        .await?;

    println!("Upload success. Version: {:?}", resp.version_id);

    let resp = client.get_object()
                .bucket("logcollectionbucket").key(data).send().await?;
    let d = resp.body.collect().await;
    println!("data: {:?}", d.unwrap().into_bytes());
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
    let client = Client::new(&shared_config); 

    ///*
    // Test upload event object
    let bucket = String::from("logcollectionbucket");
    let data = String::from("another test before stopping");
    let result = upload_object(&client,&bucket,&data).await;
    match result {
        Ok(val) => println!("{val:?}"),
        Err(err) => eprintln!("{err:?}"),
    }
    //*/

    Ok(client)
}

/*
#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// The name of the file to upload.
    #[structopt(short, long)]
    filename: String,

    /// The name of the object in the bucket.
    #[structopt(short, long)]
    key: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Upload a file to a bucket.
async fn upload_object(
    client: &Client,
    bucket: &str,
    filename: &str,
    key: &str,
) -> Result<(), Error> {
    println!("did we get here?");
    let resp = client.list_buckets().send().await?;

    for bucket in resp.buckets() {
        println!("bucket: {:?}", bucket.name().unwrap_or_default())
    }

    println!();

    let body = ByteStream::from_path(Path::new(filename)).await;

    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;

            println!("Upload success. Version: {:?}", resp.version_id);

            let resp = client.get_object().bucket(bucket).key(key).send().await?;
            let data = resp.body.collect().await;
            println!("data: {:?}", data.unwrap().into_bytes());
        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{}", e);
            process::exit(1);
        }
    }

    Ok(())
}

/// Lists your buckets and uploads a file to a bucket.
/// # Arguments
///
/// * `-b BUCKET` - The bucket to which the file is uploaded.
/// * `-k KEY` - The name of the file to upload to the bucket.
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
pub async fn start_s3() -> Result<(), Error> {

    let Opt {
        bucket,
        filename,
        key,
        region,
        verbose,
    } = Opt::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();

    if verbose {
        println!("S3 client version: {}", PKG_VERSION);
        println!(
            "Region:            {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Bucket:            {}", &bucket);
        println!("Filename:          {}", &filename);
        println!("Key:               {}", &key);
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    upload_object(&client, &bucket, &filename, &key).await
}
*/
