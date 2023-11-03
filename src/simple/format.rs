impl Solution {
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
}