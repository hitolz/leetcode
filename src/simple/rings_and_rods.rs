use std::collections::{HashMap, HashSet};

/// 2103. 环和杆
/// https://leetcode.cn/problems/rings-and-rods/description/

pub fn count_points(rings: String) -> i32 {
    let mut count = 0;
    let mut color = 0;
    let mut index = 1;

    if rings.len() < 6 {
        return 0;
    }

    let mut map: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut chars:Vec<char> = rings.chars().collect();
    while index < rings.len() {
        // R3 c = R,i = 3 map = 3,set[R]
        let c = chars[color];
        let i = chars[index].to_digit(10).unwrap() as usize;

        let value = map.get(&i);
        let mut set = if value.is_some() {
            value.unwrap().to_owned()
        } else {
            HashSet::new()
        };

        set.insert(c);
        map.insert(i, set);

        index += 2;
        color += 2;
    }

    for ele in map.values() {
        if ele.len() == 3 {
            count += 1;
        }
    }
    count
}

pub fn run() {
    let rings = "B0B6G0R6R0R6G9".to_string();
    let c = count_points(rings);
    println!("{}", c);
}
