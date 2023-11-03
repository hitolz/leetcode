/// 删除有序数组中的重复项 II
/// https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/description/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let k = 2;
    if nums.len() <= k {
        return nums.len() as i32;
    }
    let (mut slow, mut fast) = (k, k);

    while fast < nums.len() {
        println!("slow = {},falst = {},nums[fast] = {},nums[slow - k] = {}",slow,fast,nums[fast],nums[slow - k]);
        if nums[fast] != nums[slow - k] {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}

pub fn run() {
    let mut nums = vec![0,0,1,1,1,1,2,3,3]; // 输入数组

    let k = remove_duplicates(&mut nums); // 调用

    println!("{:?}", k);
    println!("{:?}", nums);
}
