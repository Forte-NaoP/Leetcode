impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, (nums.len()-1) as i32);

        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return -1
    }
}