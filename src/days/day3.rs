#[path="../common.rs"]
mod common;

pub fn day3() -> std::io::Result<()> {
    let lines = common::get_data("day3");

    // println!("{}", lines.len());
    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let li = line.split_at(line.len() / 2 as usize);
        println!("{:?}", li);
    }

    Ok(())
}