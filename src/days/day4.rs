#[path="../common.rs"]
mod common;

pub fn day4() -> std::io::Result<()> {
    let lines = common::get_data("day4");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let b = line.split(",");
        for s in b {
            println!("{:?}", s);
        }
    }
   

    Ok(())
}