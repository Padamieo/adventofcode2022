#[path="../common.rs"]
mod common;

pub fn day18() -> std::io::Result<()> {
    let lines = common::get_data("day18");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}