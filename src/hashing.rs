use std::{io};
use data_encoding::HEXLOWER;
use std::fs::File;
use std::io::{Error};
use std::path::Path;
use md5::Md5;
use sha2::{Sha256, Digest};

use crate::error::OptionError;

/// Allowed hashing algorithms
pub enum HashAlgorithm {
    SHA256,
    MD5
}

/// Checks if hash generated from file matches the documented expected hash
///
/// # Examples
///
/// ```
/// use hashing::hash_matches;
///
/// hash_matches("abcd", "abcd");
/// ```
pub fn hash_matches(file_hash: &str, correct_hash: &str) -> bool {
    file_hash == correct_hash
}

/// Hashes a supplied file (based on file path) using the selected algorithm
///
/// # Examples
///
/// ```
/// use hashing::hash_file;
///
/// hash_file("sha256", "/bob/path/to/file.iso");
/// ```
///
/// # Errors
///
/// Bubbles up errors from file_reader
/// Bubbles up errors from attempting file hashing
pub fn hash_file(hashing_method: HashAlgorithm, path: &Path) -> Result<String, Error> {
    match hashing_method {
        HashAlgorithm::SHA256 => hash_sha256(path),
        HashAlgorithm::MD5 => hash_md5(path)
    }
}

/// Hashes a file with the SHA256 algorithm
///
/// # Examples
///
/// ```
/// use hashing::{hash_sha256, file_reader};
/// let reader = file_reader("/bob/path/to/file.iso");
/// hash_sha256(reader);
/// ```
///
/// # Errors
///
/// Reader may encounter potential I/O or other errors
fn hash_sha256(path: &Path) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    let mut hasher = Sha256::new();

    let _data = io::copy(&mut file, &mut hasher)?;

    let digest = hasher.finalize();

    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hashes a file with the MD5 algorithm
///
/// # Examples
///
/// ```
/// use hashing::{hash_md5, file_reader};
/// let reader = file_reader("/bob/path/to/file.iso");
/// hash_md5(reader);
/// ```
///
/// # Errors
///
/// Reader may encounter potential I/O or other errors
fn hash_md5(path: &Path) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    let mut hasher = Md5::new();

    let _data = io::copy(&mut file, &mut hasher)?;
    let digest = hasher.finalize();

    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Select algorithm type to use for hashing file, based on CLI input
///
/// # Examples
///
/// ```
/// use algorithm_type;
///
/// algorithm_type("sha256");
/// ```
///
/// # Errors
///
/// Handle unknown algorithm choices
/// ```
/// use algorithm_type;
///
/// algorithm_type("shaw256");
/// ```
pub fn algorithm_type(method: &str) -> Result<HashAlgorithm, OptionError> {
    match method {
        "sha256" => Ok(HashAlgorithm::SHA256),
        "md5" => Ok(HashAlgorithm::MD5),
        err => Err(OptionError::new(format!("unknown action: {}", err))),
    }
}
