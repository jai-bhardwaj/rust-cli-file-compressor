use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::copy;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input> <output>", args[0]);
        return;
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    let mut gz = GzEncoder::new(&mut buffer, Compression::default());
    let start = Instant::now();
    copy(&mut file, &mut gz).unwrap();
    gz.finish().unwrap();
    println!("Compressed {} bytes to {} bytes", file.metadata().unwrap().len(), buffer.len());
    let mut file = File::create(&args[2]).unwrap();
    file.write_all(&buffer).unwrap();
    println!("Time elapsed: {:?}", start.elapsed());
}


