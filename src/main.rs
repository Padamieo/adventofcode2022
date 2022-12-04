mod day1;
use day1::day1;
mod day2;
use day2::day2;
mod day3;
use day3::day3;

fn main() -> std::io::Result<()> {
    let day:i32 = 3;
    match day {
        1 => day1(),
        2 => day2(),
        3 => day3(),
        _ => Ok(())
    };
    

    // println!("{:?} = {}", highest_arr, highest_arr.iter().sum::<i32>());

    Ok(())
}