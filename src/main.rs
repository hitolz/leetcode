use crate::simple::*;
use crate::middle::*;

mod simple;
mod middle;

fn main() {
    // let v: Vec<String> = vec![
    //     // "flower".to_string(),
    //     // "flow".to_string(),
    //     // "flight".to_string()
    //
    //     // "dog".to_string(),
    //     // "racecar".to_string(),
    //     // "car".to_string(),
    //
    //     "cir".to_string(),
    //     "car".to_string(),
    // ];
    // let x = longest_common_prefix(v);
    // println!("{}", x);

    // rings_and_rods::run();

    // let s = "12345".to_string();
    // let mut chars:Vec<char> = s.chars().collect();
    // for i in 0..s.len() {
    //     println!("{:?}",chars[i].to_digit(10).unwrap() as usize);
    // }

    length_of_last_word::run();

}