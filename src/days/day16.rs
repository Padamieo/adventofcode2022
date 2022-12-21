#[path="../common.rs"]
mod common;

pub fn day16() -> std::io::Result<()> {
    let lines = common::get_data("day16");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}