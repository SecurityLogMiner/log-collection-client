use crate::producer;
use producer::start_log_stream;


use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::primitives::Blob;
use aws_sdk_dynamodb::{config::Region, meta::PKG_VERSION, Client, Error};
use aws_sdk_dynamodb::operation::create_table::{CreateTableOutput,CreateTableError};
use crate::config::Config;
use aws_sdk_dynamodb::error::{BuildError};

use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

pub async fn
start_dynamo() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(client)
}
pub fn show_dynamodb_tables() -> Result<(), Error> {
    todo!();
    Ok(())
}

pub async fn
start_dynamodb() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(client)
}

pub async fn create_table(
    client: &Client,
    table: &str,
    key: &str,
) -> Result<CreateTableOutput, String> {
    let a_name: String = key.into();
    let table_name: String = table.into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build().unwrap();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build().unwrap();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build().unwrap();

    let create_table_response = client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await;

    match create_table_response {
        Ok(out) => {
            println!("Added table {} with key {}", table, key);
            Ok(out)
        }
        Err(e) => {
            eprintln!("Got an error creating table:{e:?}");
            Err("error".to_string())
        }
    }
}

pub async fn send_dynamodb(config: Config) {
    // Call the function to create DynamoDB table
    let dynamoclient = start_dynamodb().await;
    match dynamoclient {
        Ok(client) => {
            // check if the table listed in the configuration file
            // exists. If it does not, create it. 
            println!("{:?}",&config);
            let tables = client.list_tables()
                                .into_paginator()
                                .items()
                                .send(); 
            let table_names = tables.collect::<Result<Vec<_>,_>>().await.unwrap();
            for tbl in table_names {
                if tbl == config.dynamodb.table {
                    println!("found {tbl:?}");
                    // use the table
                    let _ = start_log_stream(config.sources.logs.clone(),
                        &client).await;
                }
            }
            if let Ok(table) = create_table(&client,
                                    "default_table",
                                    "default_key").await {
                println!("{table:?}");
            }
        },
        Err(_) => todo!(),
    }
    }
