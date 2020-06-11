use structopt::StructOpt;

use crate::hashing::HashAlgorithm;

/// Select algorithm type to use for hashing file, based on CLI input
///
/// # Examples
///
/// ```
/// use cli::algorithm_type;
///
/// algorithm_type("sha256");
/// ```
///
/// # Errors
///
/// Handle unknown algorithm choices
/// ```
/// use cli::algorithm_type;
///
/// algorithm_type("shaw256");
/// ```
pub fn algorithm_type(method: &str) -> Result<HashAlgorithm, String> {
    Ok(match method {
        "sha256" => HashAlgorithm::SHA256,
        err => return Err(format!("unknown action: {}", err))
    })
}

/// Populates CLI arguments for interacting with hashing module
#[derive(StructOpt)]
#[structopt(name = "Hashasum")]
pub struct Options {
    /// type of hashing algorithm to perform on file at inputted path
    #[structopt(short="m", long="method", possible_values=&["sha256"])]
    pub method: String,
    /// path to a local file to attempt to perform hashing on
    #[structopt(short="p", long="path", parse(from_os_str))]
    pub path: std::path::PathBuf,
    /// correct checksum for downloaded file
    #[structopt(short="e", long="expected")]
    pub expected: String
}