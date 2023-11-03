/// 移除元素
/// https://leetcode.cn/problems/remove-element/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| *x != val);
    nums.len() as i32
}

// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     let (mut slow ,mut fast) = (0,0);
//     while fast < nums.len() {
//         if nums[fast] != val {
//             nums[slow] = nums[fast];
//             slow += 1;
//         }
//         fast += 1;
//     }
//     slow as i32
// }

// 双指针法。
// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     let mut slow = 0;
//     for fast in 0..nums.len() {
//         if nums[fast] != val{
//             nums[slow] = nums[fast];
//             slow += 1;
//         }
//     }
//     slow as i32
// }

pub fn run() {
    let mut vec = vec![3, 2, 2, 3];
    let val = 3;

    // let mut vec = vec![0,1,2,2,3,0,4,2];
    // let val = 2;

    // let mut vec = vec![2];
    // let val = 3;

    let count = remove_element(&mut vec, val);
    println!("{}", count);
    println!("{:?}", vec);
}
