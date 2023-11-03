use crate::simple::Solution;

/// 删除有序数组中的重复项
/// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/description/
///
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        while l < r && r < nums.len() {
            if nums[l] != nums[r] {
                l += 1;
                nums[l] = nums[r];
            }
            r += 1;
        }
        (l + 1) as i32
    }
}

pub fn run() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]; // 输入数组

    let k = Solution::remove_duplicates(&mut nums); // 调用

    println!("{:?}", k);
    println!("{:?}", nums);
}
// 第一种解法
//impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         use std::collections::BTreeMap;
//         let mut map:BTreeMap<i32,i32> = BTreeMap::new();
//         for i in 0..nums.len() {
//             let num = &nums[i];
//             let mut count = map.get(num).unwrap_or(&0);
//             map.insert(num.to_owned(),count.clone() + 1);
//         }
//         let mut vec:Vec<i32> = map.keys().map(|k| *k).collect();
//         for (i,v) in vec.iter_mut().enumerate(){
//             nums[i] = v.clone();
//         }
//         map.keys().count() as i32
//     }
// }

// 第二种解法，其实跟第一种是一样的，都是利用了 set 去重。
//impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         use std::collections::HashSet;
//         let mut set:HashSet<i32> = HashSet::new();
//         let mut count = 0;
//         for i in 0..nums.len() {
//             let num = &nums[i];
//             if !set.contains(num){
//                 set.insert(num.clone());
//                 nums[count.to_owned()] = num.to_owned();
//                 count += 1;
//             }
//         }
//         set.len() as i32
//     }
// }

// 第三种解法，使用了 Rust 中的方法，dedup、dedup_by 都可以。
//  example
// let mut vec = vec![1, 2, 2, 3, 2];
// vec.dedup();
// assert_eq!(vec, [1, 2, 3, 2]);

// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         nums.dedup_by(|a,b| a == b);
//         nums.len() as i32
//     }
// }
