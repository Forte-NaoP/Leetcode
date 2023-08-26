impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

        use std::collections::HashSet;

        let mut nums = nums;
        nums.sort();

        let len = nums.len();

        let mut ans: HashSet<(i32, i32, i32)> = HashSet::new();

        for i in 0..len-2 {
            for j in i+1..len-1 {
                if nums[i] > 0 && nums[j] > 0 {
                    break;
                }
                if binary_search(&nums[j+1..], -(nums[i]+nums[j])) {
                    ans.insert((nums[i], nums[j], -(nums[i]+nums[j])));
                }
            }
        }

        let ans = ans.iter().map(|f| {
            vec![f.0, f.1, f.2]
        }).collect();
        ans
    }
}

fn binary_search(nums: &[i32], target: i32) -> bool {
    let len = nums.len();
    let (mut left, mut right, mut mid): (usize, usize, usize);

    left = 0;
    right = len-1;

    while left <= right {
        mid = (left + right) / 2;
        if nums[mid] > target {
            if let Some(v) = mid.checked_sub(1) {
                right = v;
            } else {
                break;
            }
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            return true;
        }
    }

    false
}