use structopt::StructOpt;

/// Populates CLI arguments for interacting with hashing module
#[derive(StructOpt)]
#[structopt(name = "Checkasum")]
pub struct Options {
    /// type of hashing algorithm to perform on file at inputted path
    #[structopt(short="m", long="method", possible_values=&["sha256", "md5"])]
    pub method: String,
    /// path to a local file to attempt to perform hashing on
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    pub path: std::path::PathBuf,
    /// correct checksum for downloaded file
    #[structopt(short = "e", long = "expected")]
    pub expected: String,
}
