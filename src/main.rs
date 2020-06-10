mod hashing;
use hashing::hash_file;
use std::process::exit;

mod cli;
use cli::Options;
// StructOpt import is required here for using its from_args method with Options
use structopt::StructOpt;
use crate::hashing::hash_matches;

fn main() {
    let args = Options::from_args();
    let hash_str = hash_file(&args.method, &args.path);

    match hash_str {
        Ok(hash) => {
            println!("{}   {}", hash, args.path.display());
            if hash_matches(&hash, &args.expected) {
                println!("SUCCESS: file hash matches expected checksum hash!")
            } else {
                println!("WARNING: File hash does not match expected checksum hash!");
                println!("Expected: {} | Actual: {}", args.expected, hash);
            }
        },
        Err(reason) => {
            println!("Problem hashing file: {}", reason);
            exit(1);
        }
    }
}
