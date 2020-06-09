use std::env;
use std::path::Path;

mod hashing;
use hashing::hash_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let hashing_method = &args[1];
    let file_path = Path::new(&args[2]);

    let hash_str = hash_file(hashing_method, file_path);
    println!("{}   {}", hash_str, file_path.display());
}