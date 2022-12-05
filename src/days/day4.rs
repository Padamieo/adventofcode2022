#[path="../common.rs"]
mod common;

pub fn day4() -> std::io::Result<()> {
    let lines = common::get_data("day4");
    let mut count = 0;

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let sections = line.split(",");
        let mut arr:[i32; 4] = [0,0,0,0];
        let mut offset = 0;
        for section in sections {
            let range = section.split("-");
            for (i, value) in range.enumerate() {
                let int = value.parse::<i32>().unwrap();
                arr[offset + i] = int;
            }
            offset = 2;
        }
        if arr[0] >= arr[2] && arr[1] <= arr[3] || arr[0] <= arr[2] && arr[1] >= arr[3] {
            count = count + 1;
        }

        // if arr[0] >= arr[2] || arr[1] <= arr[3] {
        //     println!("{:?}\n", arr);
        // }

    }
    println!("{:?}\n", count);
   
    Ok(())
}