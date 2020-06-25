use std::process::exit;
// StructOpt import is required here for using its from_args method with Options
use structopt::StructOpt;

pub mod hashing;
use hashing::{algorithm_type, hash_file, hash_matches};

mod cli;
use cli::Options;

mod error;

fn main() {
    let args = Options::from_args();
    let algorithm = match algorithm_type(&args.method) {
        Ok(algorithm) => algorithm,
        Err(reason) => {
            println!("Problem determining hashing algorithm: {}", reason);
            exit(1);
        }
    };

    let hash_str = hash_file(algorithm, &args.path);
    match hash_str {
        Ok(hash) => {
            println!("{}   {}", hash, args.path.display());
            if hash_matches(&hash, &args.expected) {
                println!("SUCCESS: File hash matches expected checksum hash!")
            } else {
                println!("WARNING: File hash does not match expected checksum hash!");
                println!("Expected: {} | Actual: {}", args.expected, hash);
            }
        }
        Err(reason) => {
            println!("Problem hashing file: {}", reason);
            exit(1);
        }
    }
}
