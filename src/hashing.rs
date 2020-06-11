use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;

/// Allowed hashing algorithms
pub enum HashAlgorithm {
    SHA256
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
pub fn hash_matches(file_hash: &str, correct_hash: &str) -> bool{
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
    let reader = file_reader(path);

    match reader {
        Ok(reader) => {
            match hashing_method {
                HashAlgorithm::SHA256 => Ok(hash_sha256(reader))
            }
        },
        Err(reason) => Err(reason)
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
fn hash_sha256<R: Read>(mut reader: R) -> String {
    // set up new context object for SHA256 Init-Update-Finish operations
    let mut context = Context::new(&SHA256);
    // maintain a buffer for reading for performance
    let mut buffer = [0; 1024];
    loop {
        match reader.read(&mut buffer) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                context.update(&buffer[..count])
            },
            Err(_reason) => println!("couldn't read file contents into buffer"),
        }
    }

    let digest = context.finish();
    HEXLOWER.encode(digest.as_ref())
}

/// Reads a file from a supplied file path, and adds it to a Buffered Reader for efficient file reading
///
/// # Examples
///
/// ```
/// use hashing::file_reader;
/// let reader = file_reader("/bob/path/to/file.iso");
/// ```
///
/// # Errors
///
/// May encounter error when attempting to open file (missing file, other I/O error)
/// ```
/// use hashing::file_reader;
/// let reader = file_reader("/bob/unknown/file/path");
/// ```
fn file_reader(path: &Path) -> Result<BufReader<File>, Error> {
    let in_file = File::open(path);

    match in_file {
        Ok(file) => Ok(BufReader::new(file)),
        Err(reason) => Err(reason)
    }
}