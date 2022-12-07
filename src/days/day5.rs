#[path="../common.rs"]
mod common;
use regex::Regex;

pub fn day5() -> std::io::Result<()> {
    let lines = common::get_data("day5");
    
    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let b = line.len();
        let est = (b)/3;
        let z = 1;
        let mut m1 = Vec::new();
        let mut m2 = vec![m1; est]; 

        let find = Regex::new(r"(\[)").unwrap();
        if find.is_match(&line) {
            for n in 0..est {
                let r = z+(n*4);
                if b < 0 {
                    break;
                }

                let ch = line.chars().nth(r).unwrap();
                if !ch.is_whitespace() {
                    m2[n].push(ch);
                    //[Z] [M] [P] [Z] [Z] [Z]
                    //
                    //2,6,10,14,18,22,
                    //0,1,2,3,4,5
                    //3,7,11,15,19,23
                    println!("  {}:{}", ch, n);
                }
            }
        }
        println!("{}::{}:{:?}\n", b, line, m2);
    }
    
    Ok(())
}