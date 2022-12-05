use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

pub fn get_data(day:&str) -> Vec<Result<String, Error>> {
    let newstr = format!("data/{}.txt", day);
    let file = File::open(newstr).expect("file not found!");
    let buf_reader = BufReader::new(file);
    let lines: Vec<_> = buf_reader.lines().collect();
    return lines;
}