// Read from a file and detect when new data is appended to that file
use std::process::Command;

/* The 'Producer' as defined in the system design file.
 *  - https://github.com/SecurityLogMiner/log-collection-client/tree/features
 * This function should ideally take a Path parameter. The goal here is to
 * read new data that has been appended to the file and send it to a 
 * Consumer.
 *
 * To test this function, [TODO insert test instructions] 
 */
pub fn run_tail_f(f: String) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    /*
    let tail_f = Command::new("tail").arg("-f").arg(f.to_string())
        .output();
    Ok(tail_f?)
    */
    let mut tail_f = Command::new("tail");
    tail_f.arg("-f");
    tail_f.arg(f.to_string());
    let res = tail_f.status().expect("failed");
    Ok(res)
}
