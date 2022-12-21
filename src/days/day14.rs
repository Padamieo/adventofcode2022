#[path="../common.rs"]
mod common;

pub fn day14() -> std::io::Result<()> {
    let lines = common::get_data("day14");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        println!("{}\n" , line);
    }
   
    Ok(())
}