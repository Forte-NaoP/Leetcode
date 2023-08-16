impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0_i64; nums.len()];
        prefix[0] = nums[0] as i64;

        for i in 1..nums.len() {
            prefix[i] = prefix[i-1] + nums[i] as i64;
        }

        let mut ans = 0;
        for i in 0..nums.len()-1 {
            if prefix[i] >= prefix[nums.len()-1] - prefix[i] {
                ans += 1;
            }
        }

        ans
    }
}