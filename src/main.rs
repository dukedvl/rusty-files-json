use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("content.txt").expect("File not found");

    file.write_all(b"shiggity")
        .expect("failed to write to file");

    Ok()
}
