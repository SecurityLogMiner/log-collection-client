use std::fs::File;
use std::io::{BufRead, BufReader};
use aws_sdk_s3::{Client};

////begin of pita////
// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
// ISSUE: when a struct member is made pub, this macro needs to match
// the changed datatype. Its a pita. Instead, or for now, just setup a
// config default struct in the set_configuration function and initialize
// values found in the config file.
/*
macro_rules! show_field_names {
    (pub struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname : $ftype),*
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
pub struct Config {
    pub server_address: String,
    pub server_port: u16,
    pub log_file_path: String,
    pub credentials: String // TLS needed
}}
*/
////end of pita////


#[derive(Debug)]
pub struct Config {
    pub server_address: String, // consider using Ipv4Addr::UNSPECIFIED
    pub server_port: String,
    pub log_paths: Vec<String>,
    pub s3_client: Option<Client>,
    pub credentials: String // TLS needed
}

pub fn
read_config(client: &Client) -> Option<Config> {
//read_config() -> Option<Config> {
    let mut fields: Vec<String> = Vec::new();
    let file = File::open("test.config").ok()?;
    let reader = BufReader::new(file);
    let mut result;
    for line in reader.lines() {
        result = line.ok()?;//.clone();
        match &result.chars().next() {
            Some(setting) => {
                if setting != &'#' {
                    fields.push(result.clone());
                }
            },
            None => continue
        }
    }
    Some(set_configuration(fields,&client))
}

// If a item in the configuration file is missing,
// return a default config and let it error out later.
// TODO: handle the error
fn
set_configuration(list: Vec<String>, client: &Client) -> Config {
    // maybe convert these to &str later on.
    let mut config = Config {
        server_address: String::from(""),
        server_port: String::from(""),
        log_paths: Vec::new(),
        credentials: String::from(""),
        s3_client: Some(client.clone()),
    };

    // TODO: clean this up somehow. just make it work for now.
    // The current solution could be to manually check for each
    // config setting, whether its length is 0, etc. 
    // Tried handling this with the macro above (see pita) but...
    // ... it was a pita.
    for item in list {
        let setting = item.split(" ").collect::<Vec<_>>();
        match setting[0] {
            "server_address" => config.server_address = setting[1].to_string(),
            "server_port" => config.server_port = setting[1].to_string(),
            "log_paths" => {
                let mut logs = item.split(' ').collect::<Vec<_>>();
                let mut paths = Vec::new();
                if !logs.is_empty() {
                    logs.remove(0); // remove the setting name
                }
                for log in logs {
                    println!("log to read: {:?}",log);
                    paths.push(log.to_string());
                }
                config.log_paths = paths.clone();
                drop(paths);
            }

            "credentials" => config.credentials = setting[1].to_string(),
            _ => continue
        }
    }   
    //println!("config: {:?}",config);// long printout with s3_client
    config
}
