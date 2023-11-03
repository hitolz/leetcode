/// 合并两个有序数组
/// https://leetcode.cn/problems/merge-sorted-array/description/
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in 0..n {
        nums1.pop();
    }
    nums1.append(nums2);
    nums1.sort();
}

pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in 0..n {
        nums1[(m + i) as usize] = nums2[i as usize];
    }
    nums1.sort()
}
fn main() {
    let mut v1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
    let mut v2 = vec![1, 2, 2];
    merge2(&mut v1, 6, &mut v2, 3);
    println!("{:?}", v1);
    // [-1, 0, 0, 1, 2, 2, 3, 3, 3]
    // [-1, 0, 0, 1, 2, 2, 3, 3, 3]
}
