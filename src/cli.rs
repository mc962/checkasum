use checkasum::hashing::HashAlgorithm;
use clap;
use clap::Parser;

/// Populates CLI arguments for interacting with hashing module
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// type of hashing algorithm to perform on file at inputted path
    #[arg(short, long, value_enum)]
    pub method: HashAlgorithm,
    /// path to a local file to attempt to perform hashing on
    #[arg(short, long, value_parser)]
    pub path: std::path::PathBuf,
    /// correct checksum for downloaded file
    #[arg(short, long)]
    pub expected: String,
}
