extern crate core;

use std::process::exit;
use checkasum::check_file_path;
// StructOpt import is required here for using its from_args method with Options
use clap::StructOpt;

pub mod hashing;

mod cli;
use cli::Options;

mod error;

fn main() {
    let args = Options::from_args();
    let result = check_file_path(&args.method, &args.path, &args.expected);

    match result {
        Ok(success_result) => {
            let actual_digest = success_result.actual_digest.unwrap_or("".to_string());
            println!("{}   {}", &actual_digest, args.path.display());
            if success_result.successful {
                println!("SUCCESS: File hash matches expected checksum hash!")
            } else {
                println!("WARNING: File hash does not match expected checksum hash!");
                println!(
                    "Expected: {} | Actual: {}",
                    success_result.expected_digest, &actual_digest
                );
                println!(
                    "Error: {}", success_result.message.unwrap_or("".to_string())
                )
            }
        }
        Err(error_result) => {
            println!(
                "Problem hashing file: {}",
                error_result.message.unwrap_or("Unknown error".to_string())
            );
            exit(1);
        }
    }
}
