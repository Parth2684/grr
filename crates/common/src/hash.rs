use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use blake3::hash;
use sha2::{Digest, Sha256};

pub fn hash_sha256<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 131072];

    loop {
        let bytes = reader.read(&mut buffer).unwrap();
        if bytes == 0 {
            break;
        }
        hasher.update(&buffer[..bytes]);
    }

    hex::encode(hasher.finalize())
}


pub fn hash_blake3(to_hash: &str) -> String {
    hash(to_hash.as_bytes()).to_string()
}