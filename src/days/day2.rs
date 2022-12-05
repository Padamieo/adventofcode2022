#[path="../common.rs"]
mod common;
use regex::Regex;

pub fn day2() -> std::io::Result<()> {
    let lines = common::get_data("day2");

    let mut acc: usize = 0;

    for sentence in lines.into_iter() {
        let line = sentence.unwrap();

        /* 
        let mut letters:[[char; 2]; 9] = [
            ['B','X'],
            ['C','Y'],
            ['A','Z'],
            ['A','X'],
            ['B','Y'],
            ['C','Z'],
            ['C','X'],
            ['A','Y'],
            ['B','Z']
        ];
        let mut fake = Regex::new("").unwrap();
        let mut regex_array:[Regex || i32; 9] = [0; 9];

        for (i, elem) in letters.iter_mut().enumerate() {
            let newstr = format!(r"({})\s({})+", elem[0], elem[1]);
            regex_array[i] = Regex::new(&newstr).unwrap();
        }
        */

        let bx = Regex::new(r"(B)\s(X)+").unwrap(); //1
        let cy = Regex::new(r"(C)\s(Y)+").unwrap(); //2
        let az = Regex::new(r"(A)\s(Z)+").unwrap(); //3
        let ax = Regex::new(r"(A)\s(X)+").unwrap(); //4
        let by = Regex::new(r"(B)\s(Y)+").unwrap(); //5
        let cz = Regex::new(r"(C)\s(Z)+").unwrap(); //6
        let cx = Regex::new(r"(C)\s(X)+").unwrap(); //7
        let ay = Regex::new(r"(A)\s(Y)+").unwrap(); //8
        let bz = Regex::new(r"(B)\s(Z)+").unwrap(); //9

        let mut e:[Regex; 9] = [bx, cy, az, ax, by, cz, cx, ay, bz];
        let mut v: usize = 0;

        for (i, elem) in e.iter_mut().enumerate() {
            if elem.is_match(&line) {
                v = i + 1;
            }
        }

        acc = acc + v;
        // println!("{}", v);
        
    }

    Ok(())
}
/*
A x = 4
A y = 8
A z = 3

B x = 1
B y = 5
B z = 9

C x = 7
C y = 2
C z = 6
 */