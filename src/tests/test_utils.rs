use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

pub fn read_file(path: &Path) -> impl Read {
    let file = File::open(path).unwrap_or_else(|error| panic!("Failed to open file: {:?}", error));

    BufReader::new(file)
}
