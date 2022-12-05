use std::{env, str::FromStr};
// use std::process;
mod days;
use days::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut day:i32 = 4;

    for arg in args.iter() {
        let day_reg = Regex::new(r"(day=)\b([1-9]|1[0-9]|2[0-5])\b").unwrap();

        if day_reg.is_match(&arg) {
            let li = arg.split_at(4);
            day = FromStr::from_str(li.1).unwrap();
        }
    }

    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        _ => day1::day1(),
    };

    // foo().unwrap_or_else(|err| println!("{:?}", err))
    // println!("{:?} = {}", highest_arr, highest_arr.iter().sum::<i32>());

    Ok(())
}