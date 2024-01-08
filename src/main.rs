mod awss3;
mod config;
mod producer;

use producer::{start_log_stream};
use config::{read_config};

// still need this to create the event and send to sink. The sink is the s3 bucket
// see producer:handle_log_data
fn main() {

    let config_data;
    config_data = read_config();

    match config_data {
        Some(config) => {
            let _ = start_log_stream(config);
        }
        None => panic!("error reading configuration. fix it.")
    }
}
