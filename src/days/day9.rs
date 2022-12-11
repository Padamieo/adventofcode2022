#[path="../common.rs"]
mod common;

pub fn day9() -> std::io::Result<()> {
    let lines = common::get_data("day9");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}