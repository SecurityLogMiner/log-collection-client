// Read from a file and detect when new data is appended to that file
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Result, Write};
use std::net::TcpStream;

/* This function should ideally take a Path parameter. The goal here is to
 * read new data that has been appended to the file and send it as a stream to
 * to the listening server socket
 *
 * To test this function, [TODO insert test instructions] 
 */
pub fn create_log_stream(f: String) -> Result<()> {
    /*
    let tail_f = Command::new("tail").arg("-f").arg(f.to_string())
        .output();
    Ok(tail_f?)
    */
    if let Ok(mut child) = Command::new("tail")
        .arg("-f")
        .arg(f.to_string())
        .stdout(Stdio::piped())
        .spawn() {
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(text) => {
                            //println!("captured line: {}", text);
                            send_stream(text);                    
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

fn send_stream(s: String) {
    //println!("{}", s.as_str());
    // TODO
    // remove hard coded IP address of the server. Intentionally put a code error here.
    // this task should be done on next project session.
    let mut stream = TcpStream::connect("44.228.113.79:44331"
        .expect("failed to connect to server");
    stream.write_all(s.as_str().as_bytes()).expect("failed to send log data");
}











