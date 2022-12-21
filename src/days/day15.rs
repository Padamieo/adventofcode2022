#[path="../common.rs"]
mod common;

pub fn day15() -> std::io::Result<()> {
    let lines = common::get_data("day15");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}