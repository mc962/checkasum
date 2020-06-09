mod hashing;
use hashing::hash_file;
use std::process::exit;

mod cli;
use cli::Options;
// StructOpt import is required here for using its from_args method with Options
use structopt::StructOpt;

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
