#[path="../common.rs"]
mod common;

fn assign_letter_value(section_one: &str, alpha: &Vec<char>) -> Vec<usize> {
    let mut value_arr: Vec<usize> = vec![0; section_one.len()];
    for (i, c) in section_one.chars().enumerate() {
        let index_element = &alpha.iter().position(|&x| x == c).unwrap();
        value_arr[i] = index_element + 1;
    }
    return value_arr;
}

pub fn day3() -> std::io::Result<()> {
    let lines = common::get_data("day3");

    let mut base_alphabet_string = String::from("");
    for letter in 'a'..='z' {
        base_alphabet_string.push(letter);
    }
    let full_alpha_str = format!("{}{}", base_alphabet_string, base_alphabet_string.to_uppercase());
    let alpha = full_alpha_str.chars().collect::<Vec<char>>();

    let mut calc = 0;

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();
        let sections = line.split_at(line.len() / 2 as usize);
        
        let mut value_arr1 = assign_letter_value(sections.0, &alpha);
        value_arr1.sort();
        value_arr1.dedup_by(|a, b| a == b);
        println!("{:?}=", value_arr1);

        let mut value_arr2 = assign_letter_value(sections.1, &alpha);
        value_arr2.sort();
        value_arr2.dedup_by(|a, b| a == b);
        println!("{:?}=", value_arr2);

        for b in value_arr1.into_iter() {
            let bb: &usize = value_arr2.iter().find(|&&x| x == b).unwrap_or_else(|| &0);
            if bb > &0 {
                println!("$$${:?}=", bb);
                calc = calc + bb;
            }
        }

    }

    println!("{:?}", calc );

    Ok(())
}