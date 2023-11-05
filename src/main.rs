// Read from a file and detect when new data is appended to that file
use std::process::Command;

fn main() {
    run_tail_f();
}

/*
 * The 'Producer' as defined in the system design file.
 *  - https://github.com/SecurityLogMiner/log-collection-backend/tree/features
 * This function should ideally take a Path parameter. The goal here is to
 * read new data that has been appended to the file and send it to a 
 * Consumer.
 *
 * To test this function, run the binary and then echo "new data" >> testfile 
 */
fn run_tail_f() {
    let mut tail_f = Command::new("tail");
    tail_f.arg("-f");
    tail_f.arg("testfile.txt");
    let res = tail_f.status().expect("failed");
    println!();
}
