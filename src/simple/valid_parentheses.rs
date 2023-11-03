/// 有效的括号
/// https://leetcode.cn/problems/valid-parentheses/description/
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' if Some('(') != stack.pop() => return false,
            ']' if Some('[') != stack.pop() => return false,
            '}' if Some('{') != stack.pop() => return false,
            _ => (),
        }
    }
    stack.is_empty()
}
fn main() {
    let s = "{()}{}".to_string();
    println!("{}", is_valid(s));
}
