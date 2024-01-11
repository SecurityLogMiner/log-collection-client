use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Result};
use std::thread;
use std::sync::mpsc::{channel,Sender,Receiver};
//use std::net::{TcpStream, SocketAddr};
use crate::config::{Config};
use aws_sdk_s3::{Client};
use crate::awss3;

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


// TODO: This function should take a channel and a sink.
// The sink is the destination for the log data. For now, it just prints to 
// stdout.
async fn 
handle_log_data(log_channel: Receiver<String>, client: &Client) {
    println!("client called");
    for log_line in log_channel {
        // rethink how to provide client, bucket, and key to this call. 
        awss3::upload_object(&log_line, client.clone());
        println!("{}", log_line);
    }
}

// for now, pass in the client,bucket,and key for handle_log_data.
pub fn 
start_log_stream(config: Config) -> Result<()> {

    let mut senders = Vec::new();
    let mut receivers = Vec::new();

    for input_log_file in config.log_paths.clone().into_iter() {
        let (sender, receiver) = channel();
        senders.push(sender);
        receivers.push(receiver);

        let sender_clone = senders.last().unwrap().clone();
        thread::spawn(move || {
            tail_and_send_log(&input_log_file, sender_clone)
                .expect("Failed to tail log file");
        });
    }

    if let Some(client) = config.s3_client {
        //println!("{client:?}");
        for (receiver, _input_log_file) in receivers.into_iter()
                .zip(config.log_paths.clone()) {
            let client_clone = client.clone().into();
            thread::spawn(move || handle_log_data(receiver, &client_clone));
        }    
    }
    
   // for (receiver, _input_log_file) in receivers.into_iter()
   //         .zip(config.log_paths.clone()) {
   //     thread::spawn(move || handle_log_data(receiver));
   // }       


    // never return
    loop {}
    Ok(()) // known unreachable.
}
