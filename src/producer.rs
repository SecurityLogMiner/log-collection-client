// Read from a file and detect when new data is appended to that file
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Result, Write};
use std::net::{TcpStream, SocketAddr};
use crate::config::{Config};

pub fn create_log_stream(config: Config) -> Result<()> {
    println!("{:?}",config.log_file_path);
    if let Ok(mut child) = Command::new("tail")
        .arg("-f")
        .arg(config.log_file_path)
        .stdout(Stdio::piped())
        .spawn() {
            if let Some(stdout) = child.stdout.take() {
                let addr = config.server_address.to_string() + 
                        ":" + &config.server_port.to_string();
                let mut stream = TcpStream::connect(&addr)
                    .expect("failed to connect to server");
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(text) => {
                            send_stream(text, &stream);                    
                        }
                        Err(err) => {
                            eprintln!("error reading the log: {}", err);
                            break;
                        }
                    }
                }
            }
            let _ = child.wait();
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other,
                                    "Error creating log stream"))
        }
}

fn send_stream(data: String, mut stream: &TcpStream) {
    stream.write_all(data.as_str().as_bytes()).expect("failed to send log data");
}
