/// 28. 找出字符串中第一个匹配项的下标
/// https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/

pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle){
        None => -1,
        Some(x) => x as i32
    }
}

pub fn run() {
    let haystack = "sadbutsad".to_string();
    let needle = "sa1d".to_string();
    let x = str_str(haystack, needle);
    println!("{}", x);
}
