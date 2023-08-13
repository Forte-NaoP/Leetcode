impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let target = target as i64;
        let mut nums = nums;
        nums.sort();

        let len = nums.len();

        let mut ans = vec![];

        for i in 0..len {
            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }
            for j in i+1..len {
                if j > i+1 && nums[j-1] == nums[j] {
                    continue;
                }
                let t_sum = nums[i] + nums[j];
                let mut l = j+1;
                let mut r = len-1;
                while l < r {
                    let sum: i64 = (nums[i] + nums[j]) as i64 + (nums[l] + nums[r]) as i64;
                    if sum == target {
                        ans.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l-1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r+1] {
                            r -= 1;
                        }
                    } else if sum > target {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                }
            }
        }

        ans
    }
}
