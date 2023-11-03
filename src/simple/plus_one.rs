/// 66.加一
/// https://leetcode.cn/problems/plus-one/description/

/// 按照加法规则，从最后一位开始加1，判断是否往前进1，进1则继续，不进1就结束了。
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut vec = digits;
    if vec.is_empty() {
        return vec![1];
    }
    let mut last = vec.len() - 1;
    let mut pre_plus = false;
    loop {
        if vec[last] == 9 {
            vec[last] = 0;
            pre_plus = true;
        } else {
            vec[last] += 1;
            pre_plus = false;
            break;
        }
        let i = last as i32 - 1;
        if i < 0 {
            break;
        }
        last -= 1;
    }
    if pre_plus {
        vec.insert(0, 1);
    }
    vec
}

pub fn plus_one2(mut digits: Vec<i32>) -> Vec<i32> {
    let mut prev_plus = false;
    for i in (0..digits.len()).rev() {
        match digits[i] {
            9 => {
                digits[i] = 0;
                prev_plus = true;
            }
            _ => {
                prev_plus = false;
                digits[i] += 1;
                break;
            }
        }
    }
    if prev_plus {
        digits.insert(0, 1);
    }
    digits
}

pub fn run() {
    let digits = vec![9, 9, 1];
    println!("{:?}", plus_one2(digits));
}
