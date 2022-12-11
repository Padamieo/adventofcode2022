#[path="../common.rs"]
mod common;

pub fn day7() -> std::io::Result<()> {
    let lines = common::get_data("day7");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}