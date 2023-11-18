use std::net::{TcpStream, TcpListener};

fn establish_connection(address: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    let socket_address = format!("{}:{}", address, port);
    let stream = TcpStream::connect(socket_address)?;
    Ok(stream)
}

fn listen_for_connections(port: u16) -> Result<TcpListener, std::io::Error> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    Ok(listener)
}
// TODO: integrate these functions with config module
// example to use in main.rs
/*
fn main() {
    match establish_connection("127.0.0.1", 8080) {
        Ok(_) => println!("Connection established successfully"),
        Err(err) => eprintln!("Error establishing connection: {}", err),
    }

    match listen_for_connections(8080) {
        Ok(_) => println!("Listening for incoming connections"),
        Err(err) => eprintln!("Error listening for connections: {}", err),
    }
}
*/

