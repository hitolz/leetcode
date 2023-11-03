/// 回文数
/// https://leetcode.cn/problems/palindrome-number/description/
pub fn is_palindrome(x: i32) -> bool {

    // return x.to_string().chars().rev().eq(x.to_string().chars())

    let binding = x.to_string();
    let c = binding.chars();
    return c.clone().rev().eq(c)
}

fn main() {
    println!("{}", is_palindrome(123));
}
