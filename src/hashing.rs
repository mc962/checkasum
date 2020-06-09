use std::path::Path;
use std::io::{BufReader, Read, Error};
use std::fs::File;
use ring::digest::{Context, SHA256};
use data_encoding::HEXLOWER;

pub fn hash_file(hashing_method: &str, path: &Path) -> Result<String, Error> {
    let reader = file_reader(path);

    match reader {
        Ok(reader) => {
            let digest  = match hashing_method {
                "sha256" => hash_sha256(reader),
                _ => panic!("unknown hashing method {}", hashing_method)
            };

            Ok(digest)
        },
        Err(reason) => Err(reason)
    }
}

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

fn file_reader(path: &Path) -> Result<BufReader<File>, Error> {
    let in_file = File::open(path);

    match in_file {
        Ok(file) => Ok(BufReader::new(file)),
        Err(reason) => Err(reason)
    }
}