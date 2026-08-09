use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use sha2::{Digest, Sha256};

pub fn hash<P: AsRef<Path>>(path: P) -> String {
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
