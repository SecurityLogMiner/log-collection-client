use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Debug)]
pub struct Config {
    pub server_address: String, // consider using Ipv4Addr::UNSPECIFIED
    pub server_port: u16,
    pub log_file_path: String,
    pub credentials: String // TLS needed
}

pub fn
read_config() -> Option<Config> {
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

        //let config_field = result.split(" ").collect::<Vec<_>>();
        //let _ = check_config_file(config_field);
        /*
        match field1 {
            Some(val) => {
                if config_field[0].to_string() !=  "#".to_string() &&
                    config_field[0].to_string() == "log_file_path".to_string() {
                        fields.push(config_field[1].to_string());
                }
            },
            None => () 
        }
        */
    }
    Some(set_configuration(fields))
}

// If a item in the configuration file is missing,
// the default value will be set for the user.
fn
set_configuration(list: Vec<String>) -> Config {
    println!("{:?}",list);
    let mut config = Config {
        server_address: String::from("0.0.0.0"),
        server_port: 44331,
        log_file_path: String::from("./test.log"),
        credentials: String::from("./creds.crt")
    };
    config
    /*
    if entry != "#".to_string() {
        if Config::field_names().contains(&entry) == true && &entry.len() > &0 {
            return Some(&entry);
        }
    }
    */
}
