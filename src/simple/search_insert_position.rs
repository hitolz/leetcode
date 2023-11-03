/// 35. 搜索插入位置
/// https://leetcode.cn/problems/search-insert-position/description

/// 暴力解法
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 || nums[0] > target {
        return 0;
    }
    if nums[nums.len() - 1] < target {
        return (nums.len()) as i32;
    }
    for i in 0..nums.len() {
        if nums[i] < target {
            continue;
        }
        if nums[i] == target {
            return i as i32;
        }
        if nums[i] > target {
            return i as i32;
        }
        if nums[i + 1] > target {
            return (i + 1) as i32;
        }
    }
    (nums.len()) as i32
}

/// 二分查找法
pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
    return match nums.binary_search(&target) {
        Ok(i) => i as i32,
        Err(i) => i as i32,
    };
}

/// 手写二分查找
pub fn search_insert3(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as i32;
    let mut right = nums.len() as i32 - 1;
    let mut ans = nums.len() as i32;
    while left <= right {
        let mid = (left + right) / 2 ;
        if nums[mid as usize] >= target {
            right = mid - 1;
            ans = mid;
        } else {
            left = mid + 1;
        }
    }
    return ans;
}

pub fn run() {
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let index = search_insert3(nums, target);
    println!("{}", index);
}
