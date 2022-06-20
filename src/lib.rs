use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod hashing;
use crate::hashing::{hash_file, HashAlgorithm};
use hashing::{algorithm_type, hash_file_path, hash_matches};

pub mod error;

/// Results after hashing contents of a file
#[derive(Serialize, Deserialize, Debug)]
pub struct ChecksumResult {
    pub expected_digest: String,
    pub actual_digest: Option<String>,
    pub hashing_algorithm: String,
    pub successful: bool,
    pub message: Option<String>,
}

fn process_result(method: &HashAlgorithm, expected: &str, hash_str: Result<String, Error>) -> ChecksumResult {
    let result = match hash_str {
        Ok(hash) => {
            if hash_matches(&hash, expected) {
                ChecksumResult {
                    expected_digest: expected.to_string(),
                    actual_digest: Some(hash.to_string()),
                    hashing_algorithm: method.to_string(),
                    successful: true,
                    message: None,
                }
            } else {
                ChecksumResult {
                    expected_digest: expected.to_string(),
                    actual_digest: Some(hash.to_string()),
                    hashing_algorithm: method.to_string(),
                    successful: false,
                    message: None,
                }
            }
        }
        Err(reason) => ChecksumResult {
            expected_digest: expected.to_string(),
            actual_digest: None,
            hashing_algorithm: method.to_string(),
            successful: false,
            message: Some(reason.to_string()),
        },
    };

    return result;
}

/// Checks file at given path, comparing against an expected checksum digest, using a particular
/// matching hashing method.
///
/// # Examples
///
/// ```
/// use std::env;
/// use std::path::PathBuf;
/// use checkasum::check_file_path;
///
/// let sample_file_path: PathBuf = [
///             env::var("CARGO_MANIFEST_DIR").unwrap(),
///             "tests".to_string(),
///             "fixtures".to_string(),
///             "sample.txt".to_string()
///         ].iter().collect();
///
/// let _result = check_file_path("sha256", &sample_file_path, &"test_digest".to_string());
/// ```
pub fn check_file_path(
    method: &str,
    path: &PathBuf,
    expected: &str,
) -> Result<ChecksumResult, ChecksumResult> {
    return match algorithm_type(method) {
        Ok(algorithm) => {
            let hash_str = hash_file_path(&algorithm, path);

            let result = process_result(&algorithm, expected, hash_str);

            Ok(result)
        },
        Err(reason) => {
            Ok(ChecksumResult {
                expected_digest: expected.to_string(),
                actual_digest: None,
                hashing_algorithm: method.to_string(),
                successful: false,
                message: Some(reason.to_string()),
            })
        }
    }
}

/// Checks uploaded file, comparing against an expected checksum digest, using a particular
/// matching hashing method.
///
/// # Examples
///
/// ```
/// use std::env;
/// use std::fs::File;
/// use std::path::PathBuf;
/// use checkasum::check_file;
///
/// let mut file = File::create("tests/fixtures/tmp/foo.txt").unwrap();
///
/// let _result = check_file("sha256", &mut file, &"test_digest".to_string());
/// ```
pub fn check_file(
    method: &str,
    infile: &mut File,
    expected: &str,
) -> Result<ChecksumResult, ChecksumResult> {
    return match algorithm_type(method) {
        Ok(algorithm) => {
            let hash_str = hash_file(&algorithm, infile);

            let result = process_result(&algorithm, expected, hash_str);

            Ok(result)
        },
        Err(reason) => {
            Ok(ChecksumResult {
                expected_digest: expected.to_string(),
                actual_digest: None,
                hashing_algorithm: method.to_string(),
                successful: false,
                message: Some(reason.to_string()),
            })
        }
    };


}
