use std::io::Error;
#[path="../common.rs"]
mod common;

fn analyze_length (lines: &Vec<Result<String, Error>>) -> i32 {
    let mut x: i32 = 0;
    let mut r: i32 = 0;
    for sentence in lines.into_iter() {
        match sentence.as_ref().unwrap().parse::<i32>() {
            Ok(_e)=>{
                r = r + 1;
            },
            Err(_e) => {
                if r > x {
                    x = r;
                }
                r = 0;
            }
        }
    }
    return x;
}

pub fn day1() -> std::io::Result<()> {
    let lines = common::get_data("day1");
    let length = analyze_length(&lines);
    let mut arr_vec = vec![0; length as usize];
    // let mut highest = 0;
    let mut highest_arr: Vec<i32> = vec![0; 3];
    let mut v: usize = 0;
    for sentence in lines.into_iter() {
        match sentence.unwrap().parse::<i32>() {
            Ok(x)=>{
                arr_vec[v] = x;
                v = v + 1;
            },
            Err(_e) => {
                let mut calc: i32 = 0;
                for elem in arr_vec.iter_mut() {
                    calc = calc + *elem;
                }

                if calc > highest_arr[0] {
                    highest_arr[2] = highest_arr[1];
                    highest_arr[1] = highest_arr[0];
                    highest_arr[0] = calc;
                } else if calc > highest_arr[1] {
                    highest_arr[2] = highest_arr[1];
                    highest_arr[1] = calc;
                } else if calc > highest_arr[2] {
                    highest_arr[2] = calc;
                }
                v = 0;
                for elem in arr_vec.iter_mut() { *elem = 0; }
            }
        }
        // println!("{}", b);
    }

    println!("{:?} = {}", highest_arr, highest_arr.iter().sum::<i32>());

    Ok(())
}