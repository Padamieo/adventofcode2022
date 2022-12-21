#[path="../common.rs"]
mod common;
use std::io::Error;

use regex::Regex;

fn define_devision (lines: &Vec<Result<String, Error>>) -> i32 {
    let mut divide: i32 = 0;
    for sentence in lines.into_iter() {
        if divide < 0 {
            break;
        }
        let line = sentence.as_ref().unwrap();
        let line_length = line.len() as i32;
        divide = (line_length)/3;
    }
    return divide;
}

pub fn day5() -> std::io::Result<()> {
    let lines = common::get_data("day5");
    let devision = define_devision(&lines);
    let mut m1: Vec<char> = Vec::new();
    let mut m2 = vec![m1; devision as usize];

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let line_length = line.len() as i32;
        let z = 1;

        let find = Regex::new(r"(\[)").unwrap();
        if find.is_match(&line) {
            for n in 0..devision {
                let r = z+(n*4);
                if line_length < 0 {
                    break;
                }

                let ch = line.chars().nth(r).unwrap();
                if !ch.is_whitespace() {
                    m2[n as usize].push(ch);
                    //[Z] [M] [P] [Z] [Z] [Z]
                    //
                    //2,6,10,14,18,22,
                    //0,1,2,3,4,5
                    //3,7,11,15,19,23
                    println!("  {}:{}", ch, n);
                }
            }
        }
        println!("{}::{}:{:?}\n", line_length, line, m2);
    }
    
    Ok(())
}