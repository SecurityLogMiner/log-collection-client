use std::fs::File;
use std::io::{BufRead, BufReader};

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
    pub log_file_path: String,
    pub dynamo_table_name: String,
}}
*/
////end of pita////


#[derive(Debug, Clone)]
pub struct Config {
    pub log_paths: Vec<String>,
    pub dynamo_table_name: String, 
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
    }
    Some(set_configuration(fields))
}

// If a item in the configuration file is missing,
// return a default config and let it error out later.
// TODO: handle the error
fn
set_configuration(list: Vec<String>) -> Config {
    // maybe convert these to &str later on.
    let mut config = Config {
        log_paths: Vec::new(),
        dynamo_table_name: String::from(""),
    };

    // TODO: clean this up somehow. just make it work for now.
    // The current solution could be to manually check for each
    // config setting, whether its length is 0, etc. 
    // Tried handling this with the macro above (see pita) but...
    // ... it was a pita.
    for item in list {
        let setting = item.split(" ").collect::<Vec<_>>();
        match setting[0] {
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

            "dynamo_table_name" => config.dynamo_table_name = setting[1].to_string(),
            _ => continue
        }
    }   
    config
}
