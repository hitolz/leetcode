/// 最长公众前缀
/// https://leetcode.cn/problems/longest-common-prefix/description/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    let min_len = min_len(&strs);

    for i in 0..min_len {
        if !all_eq(&strs, i) {
          return result; 
        }
        let ch = strs[0].chars().nth(i as usize).unwrap();
        result.push(ch);
    }

    return result;
}

fn all_eq(strs: &Vec<String>, index: usize) -> bool {
    let len = strs.len();
    let ch = strs[0].chars().nth(index);
    for i in 1..len{
        if strs[i].chars().nth(index) != ch {
            return false;
        }
    }
    return true;
}

fn min_len(strs: &Vec<String>) -> usize {
    let mut min = 0;
    for i in 0..strs.len() {
        if min < strs[i].len() {
            min = strs[i].len();
        }
    }
    return min;
}

fn main() {
    let v: Vec<String> = vec![
        // "flower".to_string(),
        // "flow".to_string(),
        // "flight".to_string()
        
        // "dog".to_string(),
        // "racecar".to_string(),
        // "car".to_string(),

        "cir".to_string(),
        "car".to_string(),
    ];
    let x = longest_common_prefix(v);
    println!("{}", x);
}
