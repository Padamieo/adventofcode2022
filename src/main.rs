use std::{env, str::FromStr};
mod days;
use days::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut day:i32 = 7;

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
        5 => day5::day5(),
        6 => day6::day6(),
        7 => day7::day7(),
        8 => day8::day8(),
        9 => day9::day9(),
        10 => day10::day10(),
        11 => day11::day11(),
        12 => day12::day12(),
        13 => day13::day13(),
        14 => day14::day14(),
        15 => day15::day15(),
        16 => day16::day16(),
        17 => day17::day17(),
        18 => day18::day18(),
        19 => day19::day19(),
        20 => day20::day20(),
        21 => day21::day21(),
        // 22 => day22::day22(),
        // 23 => day23::day23(),
        // 24 => day24::day24(),
        // 25 => day25::day25(),
        _ => day1::day1(),
    };

    // foo().unwrap_or_else(|err| println!("{:?}", err))
    // println!("{:?} = {}", highest_arr, highest_arr.iter().sum::<i32>());

    Ok(())
}