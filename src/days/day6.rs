#[path="../common.rs"]
mod common;
use std::{hash::Hash, collections::HashSet};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn day6() -> std::io::Result<()> {
    let lines = common::get_data("day6");

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let chars_array = line.chars().collect::<Vec<char>>();
        let mut position = 0;

        for (i, _el) in chars_array.iter().enumerate() {
            if position != 0 {
                break;
            }

            if i > 3 {
                if has_unique_elements(
                    [chars_array[i-4], chars_array[i-3], chars_array[i-2], chars_array[i-1]]
                ) {
                    position = i;
                }
            }
        }

        println!("{}\n" , position);
    }
   
    Ok(())
}