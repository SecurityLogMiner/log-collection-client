use aws_sdk_kinesis::primitives::Blob;
use aws_sdk_kinesis::{config::Region, meta::PKG_VERSION, Client, Error};
use aws_sdk_kinesis::operation::list_streams;

pub async fn
start_elastic() -> Result<(), Error> {
    Ok(())
}

pub async fn
start_kinesis() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn 
show_streams(client: &Client
                      ) -> Result<(), Error> {
    let resp = client.list_streams().send().await?;

    println!("Stream names:");

    let streams = resp.stream_names;
    for stream in &streams {
        println!("  {}", stream);
    }

    println!("Found {} stream(s)", streams.len());

    Ok(())
}

pub async fn 
add_record(client: &Client, stream: &str, key: &str, data: &str
           ) -> Result<(), Error> {
    let blob = Blob::new(data);

    client
        .put_record()
        .data(blob)
        .partition_key(key)
        .stream_name(stream)
        .send()
        .await?;

    println!("Put data into stream.");

    Ok(())
}
