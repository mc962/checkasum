mod hashing;
use hashing::hash_file;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Hashasum")]
struct Options {
    /// type of hashing algorithm to perform on file at inputted path
    #[structopt(short = "m", long = "method")]
    method: String,
    /// path to a local file to attempt to perform hashing on
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Options::from_args();
    let hash_str = hash_file(&args.method, &args.path);

    match hash_str {
        Ok(hash) => println!("{}   {}", hash, args.path.display()),
        Err(reason) => {
            println!("Problem hashing file: {}", reason);
            exit(1);
        }
    }
}
