use std::fs::File;
use std::io::{BufRead, BufReader};

// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
macro_rules! show_field_names {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}

show_field_names!{
struct Config {
    server_address: String,
    server_port: u16,
    log_file_path: String,
    field_values: Vec<String>,
    credentials: String // TLS needed
    db_host: String,
    db_port: u16,
}}

pub fn
read_config() -> Result<u8, Box<dyn std::error::Error>> {
    let file = File::open("test.config")?;
    let reader = BufReader::new(file);
    let mut result;
    for line in reader.lines() {
        result = line?.clone();
        let config_field = result.split(" ").collect::<Vec<_>>();
        println!("config_field: {:?}", config_field);
        let field1 = &check_config_file(config_field[0]);
        match field1 {
            Some(val) => println!("config member: {:?}, file data: {:?}",
                                  val,
                                  config_field[0].to_string() == val.to_string()),
            None => {} 
        }
    }
    /*
     * TODO read the config entries and establish connection with server*/
    Ok(0)
}

/*
 * Check the configuration file for proper format
 * */
fn
check_config_file(entry: &str) -> Option<&str> {
    if entry != "#".to_string() {
        if Config::field_names().contains(&entry) == true && &entry.len() > &0 {
            //println!("{:?} -- {:?} -- {}", Config::field_names().contains(&entry), &entry, &entry.len());
            return Some(&entry);
        }
    }
    None
}

//still testing with and messing around with this code.


/*fn listen_for_connections(port: u16) -> Result<(), String> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).map_err(|e| e.to_string())?;

    println!("Server listening on port {}", port);

    // Accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                
                // Example: Read data from the connection
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        // Echo the data back to the client
                        stream.write_all(&buffer[..size]).unwrap();
                    }
                    Err(e) => {
                        println!("Failed to read from connection: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}*/











// UNUSED Section
/*
#[derive(Parser,Debug)]
struct Args {
    config: String,
}
// the user could provide a path to a different config or simply get help with using the command
pub fn 
command_line() -> Result<Config, Box<dyn std::error::Error>>{
    let args = Args::try_parse();
    println!("{:?}",args);
    Ok(Config {
        server_address: String::from("server address"),
        server_port: 123,
        log_file_path: String::from("path to log file"),
        field_values: vec!["test".to_string(), "field".to_string(), "values".to_string()],
        credentials: String::from("credentials"),
    })
}
*/

