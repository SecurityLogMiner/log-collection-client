use std::process::{Command, Stdio};
use tokio::time;
use ctrlc;
use async_trait::async_trait;
use std::fs::{File,OpenOptions};
use std::iter::zip;
use std::io::{BufReader, BufRead, Write, Result};
use std::thread;
use std::sync::mpsc::{channel,Sender,Receiver};
use uuid::Uuid;
use crate::config::{Config};
use crate::dynamosdk;
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Debug, Clone)]
pub struct DataBuffer {
    name: String,
}

// The trait and impl will need to move to the dynamosdk module. 
#[async_trait]
trait TestDynamo {
    fn show(&self) -> String;
    async fn handle_log_data(&self,log_channel: Receiver<String>);
}
#[async_trait]
impl TestDynamo for DynamodbClient {
    fn show(&self) -> String {
        format!("{self:?}")
    }

    async fn handle_log_data(&self, log_channel: Receiver<String>) {
        if let Ok(table) = self.describe_table().table_name("eptesttable").send().await {
            for log_line in log_channel {
                println!("{log_line}");
                let res = self.put_item()
                    .table_name("eptesttable")
                    .item("epkeyitem",AttributeValue::S(log_line))
                    .send().await;
                // dumb error checking for now. eventually, this will need to be 
                // sent to the status api for the user.
                //println!("{res:?}");
            }
        }
    }
}

fn 
tail_and_send_log(path: &str, sender: Sender<String>) -> Result<()> {
    let mut tail_process = Command::new("tail")
        .args(["-f","-n0","-q", &path])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = tail_process.stdout.take().expect("Failed to open stdout");

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                sender.send(line).expect("Failed to send data");
            }
        }
    });
    Ok(())
}

pub fn create_data_buffer() -> Result<DataBuffer> {
    let uuid = Uuid::new_v4();
    let _ = File::create(uuid.to_string())?;
    let mut bf = DataBuffer {
        name: uuid.to_string(),
    };
    Ok(bf)
}

pub fn destroy_data_buffer(name: String) -> Result<()> {
    std::fs::remove_file(name)?;
    Ok(())
}


pub fn insert_into_buffer(mut bf: File, data: &str) -> Result<()> {
    bf.write_all(b"some data should be in amihere.txt")?;
    Ok(())
}

pub async fn 
start_log_stream(paths: Vec<String>) -> Result<()> {
    let (tx,rx) = channel();
    ctrlc::set_handler(move || {
        println!("handle ctrlc signal");
        tx.send(()).expect("unable to send termination signal");
    }).expect("issue with ctrlc signal handling");
    
    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    let mut clients = Vec::<_>::new();
    let mut log_count = 0;

    for input_log_file in paths.clone().into_iter() {
        log_count += 1;
        if let Ok(client) = dynamosdk::start_dynamodb().await {
            clients.push(client);
        }

        let (sender, receiver) = channel();
        senders.push(sender);
        receivers.push(receiver);
         
        let sender_clone = senders.last().unwrap().clone();
        thread::spawn(move || {
            tail_and_send_log(&input_log_file, sender_clone)
                .expect("Failed to tail log file");
        });
    }

    let iter = zip(receivers.into_iter(), clients);
    for (receiver, client) in iter {
        thread::spawn(move || {
            let tokio_handle = tokio::runtime::Runtime::new().unwrap();
                tokio_handle.block_on(async {
                    client.handle_log_data(receiver).await;
                });
        });
    }

    rx.recv().expect("unable to receive from channel");

    Ok(())
}


#[test]
fn test_the_thing() {
    assert_eq!(1,1);
}
