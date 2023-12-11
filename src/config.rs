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
}}

pub fn
read_config() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut fields: Vec<String> = Vec::new();
    let file = File::open("test.config")?;
    let reader = BufReader::new(file);
    let mut result;
    for line in reader.lines() {
        result = line?.clone();
        let config_field = result.split(" ").collect::<Vec<_>>();
        let field1 = &check_config_file(config_field[0]);
        match field1 {
            Some(val) => {
                if config_field[0].to_string() !=  "#".to_string() &&
                    config_field[0].to_string() == "log_file_path".to_string() {
                        fields.push(config_field[1].to_string());
                }
            },
            None => {} 
        }
    }
    /*
     * TODO read the config entries and establish connection with server*/
    Ok(fields)
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






































