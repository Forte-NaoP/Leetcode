impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        if nums.len() > 1 {
            dp[1] = max(dp[0], nums[1]);
        }

        for i in 2..nums.len() {
            dp[i] = max(dp[i-1], dp[i-2] + nums[i]);
        }

        dp[nums.len()-1]
    }
}