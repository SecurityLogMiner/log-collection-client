use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Result};
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{channel,Sender,Receiver};
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


async fn 
handle_log_data(log_channel: Receiver<String>, client: Client) {
    // can retrieve a thread name from the REceiever
    println!("client called");
    for log_line in log_channel {
        println!("{}", log_line);
        awssdk::add_record(&client,"the things","datakey",&log_line).await;
    }
}

pub async fn 
start_log_stream(config: Config) -> Result<()> {

    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    let mut clients = Vec::<Client>::new();

    for input_log_file in config.log_paths.clone().into_iter() {
        if let Ok(client) = awssdk::start_kinesis().await {
            clients.push(client);
        }
        let (sender, receiver) = channel();
        senders.push(sender);
        receivers.push(receiver);
         
        let sender_clone = senders.last().unwrap().clone();
        thread::spawn(move || {
            // might need Arc for client
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
                handle_log_data(receiver,client).await;
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
