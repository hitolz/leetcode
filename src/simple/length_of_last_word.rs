///58. 最后一个单词的长度
/// https://leetcode.cn/problems/length-of-last-word/description/

pub fn length_of_last_word(s: String) -> i32 {
    let mut len = 0;
    let mut start = false;
    for ch in s.chars().rev().into_iter() {
        match ch{
            ' ' => {
                if start {
                    break;
                }                
            },
            _ =>{
                start = true;
                len += 1;
            }
        }
    }
    len
}

pub fn run() {
    let s = "luffyisstilljoyboy".to_string();
    let len = length_of_last_word(s);
    println!("{}", len);
}
