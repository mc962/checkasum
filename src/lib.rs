use std::path::PathBuf;

use serde::{Serialize, Deserialize};

pub mod hashing;
use hashing::{algorithm_type, hash_file, hash_matches};

pub mod error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChecksumResult {
    pub expected_digest: String,
    pub actual_digest: Option<String>,
    pub hashing_algorithm: String,
    pub successful: bool,
    pub message: Option<String>
}

pub fn check_file(method: &str, path: &PathBuf, expected: &str) -> Result<ChecksumResult, ChecksumResult> {
    let algorithm = match algorithm_type(method) {
        Ok(algorithm) => {
            algorithm
        }
        Err(reason) => {
            return Ok(ChecksumResult{
                expected_digest: expected.to_string(),
                actual_digest: None,
                hashing_algorithm: method.to_string(),
                successful: false,
                message: Some(reason.to_string())
            })
        }
    };

    let hash_str = hash_file(algorithm, path);
    let result = match hash_str {
        Ok(hash) => {
            if hash_matches(&hash, expected) {
                ChecksumResult{
                    expected_digest: expected.to_string(),
                    actual_digest: Some(hash.to_string()),
                    hashing_algorithm: method.to_string(),
                    successful: true,
                    message: None
                }
            } else {
                ChecksumResult{
                    expected_digest: expected.to_string(),
                    actual_digest: Some(hash.to_string()),
                    hashing_algorithm: method.to_string(),
                    successful: false,
                    message: None
                }
                // println!("WARNING: File hash does not match expected checksum hash!");
                // println!("Expected: {} | Actual: {}", expected, hash);
            }
        }
        Err(reason) => {
            ChecksumResult{
                expected_digest: expected.to_string(),
                actual_digest: None,
                hashing_algorithm: method.to_string(),
                successful: false,
                message: Some(reason.to_string())
            }
        }
    };

    return Ok(result)
}