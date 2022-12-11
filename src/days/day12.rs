#[path="../common.rs"]
mod common;

pub fn day12() -> std::io::Result<()> {
    let lines = common::get_data("day12");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}