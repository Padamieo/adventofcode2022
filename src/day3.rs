use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn day3() -> std::io::Result<()> {
    let file = File::open("day3.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);
    let lines: Vec<_> = buf_reader.lines().collect();

    println!("{}", lines.len());

    Ok(())
}