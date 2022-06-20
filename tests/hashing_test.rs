use checkasum::{check_file, check_file_path};
use std::env;
use std::fs::File;
use std::path::PathBuf;

struct Setup {
    valid_checksum: String,
    sample_file_path: PathBuf,
}

impl Setup {
    fn new() -> Self {
        let sample_file_path: PathBuf = [
            env::var("CARGO_MANIFEST_DIR").unwrap(),
            "tests".to_string(),
            "fixtures".to_string(),
            "sample.txt".to_string(),
        ]
        .iter()
        .collect();

        Self {
            valid_checksum: "c5e5e387b962dcc4a87bbd299babb8eee70d452a0fb62513d67bea8649bd73cd"
                .to_string(),
            sample_file_path,
        }
    }
}

#[test]
fn valid_file_path_checksum_matches() {
    let setup = Setup::new();
    let result = check_file_path("sha256", &setup.sample_file_path, &setup.valid_checksum).unwrap();

    assert_eq!(result.successful, true);
    assert_eq!(result.expected_digest, result.actual_digest.unwrap());
}

#[test]
fn invalid_file_path_checksum_does_not_match() {
    let setup = Setup::new();
    let invalid_checksum = setup.valid_checksum + "-invalid";
    let result = check_file_path("sha256", &setup.sample_file_path, &invalid_checksum).unwrap();

    assert_eq!(result.successful, false);
    assert_ne!(result.expected_digest, result.actual_digest.unwrap());
}

#[test]
fn valid_file_checksum_matches() {
    let setup = Setup::new();
    let mut sample_file = File::open(setup.sample_file_path).unwrap();
    let result = check_file("sha256", &mut sample_file, &setup.valid_checksum).unwrap();

    assert_eq!(result.successful, true);
    assert_eq!(result.expected_digest, result.actual_digest.unwrap());
}

#[test]
fn invalid_file_checksum_does_not_match() {
    let setup = Setup::new();
    let invalid_checksum = setup.valid_checksum + "-invalid";
    let mut sample_file = File::open(setup.sample_file_path).unwrap();
    let result = check_file("sha256", &mut sample_file, &invalid_checksum).unwrap();

    assert_eq!(result.successful, false);
    assert_ne!(result.expected_digest, result.actual_digest.unwrap());
}
