use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Hashasum")]
pub struct Options {
    /// type of hashing algorithm to perform on file at inputted path
    #[structopt(short = "m", long = "method")]
    pub method: String,
    /// path to a local file to attempt to perform hashing on
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    pub path: std::path::PathBuf
}