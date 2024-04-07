extern crate core;

use clap::Parser;
use std::process::exit;

use crate::cli::Options;
use checkasum::check_file_path;

pub mod hashing;

mod cli;
mod error;

fn main() {
    let args = Options::parse();
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

                let additional_error_information = success_result.message.unwrap_or("".to_string());
                if !additional_error_information.is_empty() {
                    println!("Error: {}", additional_error_information)
                }
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
