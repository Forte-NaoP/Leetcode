impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let len = nums.len();

        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp = vec![0; nums.len()];
        let mut dp2 = vec![0; nums.len()];

        dp[0] = nums[0];
        dp[1] = nums[0];
        dp2[1] = nums[1];

        for i in 2..len-1 {
            dp[i] = max(dp[i-1], dp[i-2] + nums[i]);
        }

        for i in 2..len {
            dp2[i] = max(dp2[i-1], dp2[i-2] + nums[i]);
        }

        max(dp[nums.len()-2], dp2[nums.len()-1])

    }
}