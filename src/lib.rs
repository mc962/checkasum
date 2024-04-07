use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use hashing::{hash_file_path, hash_matches};

use crate::hashing::{hash_file, HashAlgorithm};

pub mod error;
pub mod hashing;

/// Results after hashing contents of a file
#[derive(Serialize, Deserialize, Debug)]
pub struct ChecksumResult {
    pub expected_digest: String,
    pub actual_digest: Option<String>,
    pub hashing_algorithm: String,
    pub successful: bool,
    pub message: Option<String>,
}

/// Processes String created after hashing file into a result struct expected by external users
fn process_result(
    method: &HashAlgorithm,
    expected: &str,
    hash_str: Result<String, Error>,
) -> ChecksumResult {
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
/// use checkasum::hashing::HashAlgorithm::SHA256;
///
/// let sample_file_path: PathBuf = [
///             env::var("CARGO_MANIFEST_DIR").unwrap(),
///             "tests".to_string(),
///             "fixtures".to_string(),
///             "sample.txt".to_string()
///         ].iter().collect();
///
/// let _result = check_file_path(&SHA256, &sample_file_path, &"test_digest".to_string());
/// ```
pub fn check_file_path(
    method: &HashAlgorithm,
    path: &PathBuf,
    expected: &str,
) -> Result<ChecksumResult, ChecksumResult> {
    let hash_str = hash_file_path(method, path);
    let result = process_result(method, expected, hash_str);

    Ok(result)
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
/// use checkasum::hashing::HashAlgorithm::SHA256;
///
/// let mut file = File::create("tests/fixtures/tmp/foo.txt").unwrap();
///
/// let _result = check_file(&SHA256, &mut file, &"test_digest".to_string());
/// ```
pub fn check_file(
    method: &HashAlgorithm,
    infile: &mut File,
    expected: &str,
) -> Result<ChecksumResult, ChecksumResult> {
    let hash_str = hash_file(method, infile);

    let result = process_result(method, expected, hash_str);

    Ok(result)
}
