#[path="../common.rs"]
mod common;

pub fn day5() -> std::io::Result<()> {
    let lines = common::get_data("day5");
    
    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let b = line.len();
        let est = (b)/3;
        let mut sum = 0;
        
        for n in 0..est {
            let ch = line.chars().nth(0).unwrap();
            println!(":{}", n);
            sum += n;
        }
        println!("{}:{:?}\n", sum, line);
    }
    
    Ok(())
}