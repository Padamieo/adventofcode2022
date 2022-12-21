#[path="../common.rs"]
mod common;

pub fn day23() -> std::io::Result<()> {
    let lines = common::get_data("day23");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}