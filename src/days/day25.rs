#[path="../common.rs"]
mod common;

pub fn day25() -> std::io::Result<()> {
    let lines = common::get_data("day25");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}