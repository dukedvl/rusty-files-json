pub mod reading;

use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let file_ptr = File::open("100mb.json").unwrap();

    let reader = BufReader::new(file_ptr);

    println!("file loaded in : {} ms", now.elapsed().as_millis());

    let _json: serde_json::Value = serde_json::from_reader(reader).unwrap();

    println!("file stored as json in: {} ms", now.elapsed().as_millis(),);

    println!("total time: {}", now.elapsed().as_millis());
}
