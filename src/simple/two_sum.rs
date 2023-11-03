/// 两数之和
/// https://leetcode.cn/problems/two-sum/

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let result = Vec::new();
    if nums.is_empty(){
        return result;
    }
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for i in 0..nums.len() {
        let num: i32 = target - nums[i];
        if map.contains_key(&num){
            return vec![map.get(&num).unwrap().to_owned(),i as i32];
        }
        map.insert(nums[i], i as i32);
    }
    return result;
}


fn main() {
    let nums = vec![3,3];
    let target = 6;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}
