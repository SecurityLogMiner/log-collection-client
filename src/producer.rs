use std::process::{Command, Stdio};
use std::fs::File;
use std::iter::zip;
use std::io::{BufReader, BufRead, Write, Result};
use std::thread;
use std::sync::mpsc::{channel,Sender,Receiver};
use uuid::Uuid;
use crate::config::{Config};
use crate::awssdk;
use aws_sdk_kinesis::{Client};

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

pub fn create_data_buffer() -> Result<File> {
    let uuid = Uuid::new_v4();
    let mut bf = File::create(uuid.to_string())?;
    Ok(bf)
}


pub fn insert_into_buffer(mut bf: File, data: &str) -> Result<()> {
    bf.write_all(b"some data should be in amihere.txt")?;
    Ok(())
}

pub fn send_data_buffer() {
    todo!();
}

async fn 
handle_log_data(log_channel: Receiver<String>, client_buffer: Client) {
    println!("client called");
    println!("{:?}",log_channel);
    //println!("{:?}",bf);
    for log_line in log_channel {
        println!("{}", log_line);
    }
}

pub async fn 
start_log_stream(config: Config) -> Result<()> {

    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    let mut buffers = Vec::<File>::new();
    let mut clients = Vec::<Client>::new();

    let mut a = Vec::<u32>::new();
    let mut b = Vec::<u32>::new();
    for i in 1..10 {
        a.push(i);
        b.push(i+10);
    }

    let mut z = zip(a,b);
    println!("{:?}", z.next().unwrap());

    for input_log_file in config.log_paths.clone().into_iter() {
        // replace this with start_firehose().await. 
        if let Ok(client) = awssdk::start_kinesis().await {
            clients.push(client);
        }

        if let Ok(bf) = create_data_buffer() {
            buffers.push(bf);
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

    let mut count: u8 = 0;
    for (receiver, client) in receivers.into_iter().zip(clients) {
        count += 1;
        println!("called {count} time(s)");
        thread::spawn(move || {
            let tokio_handle = tokio::runtime::Runtime::new().unwrap();
                tokio_handle.block_on(async {
                    // the file buffer needs to gt passed into this as well
                    // todo!
                    handle_log_data(receiver, client).await;
                });
        });
    }
    // never return
    loop {}
    Ok(()) // known unreachable.
}

#[test]
fn test_the_thing() {
    assert_eq!(1,1);
}
