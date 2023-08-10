impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {

        use std::collections::HashMap;
        let mut ans = 0;
        let mut sum = 0;
        let mut counter = HashMap::new();
        counter.insert(0, 1);

        for num in nums.iter() {
            sum += *num;

            if let Some(v) = counter.get(&(sum-k)) {
                ans += *v;
            }

            *counter.entry(sum).or_insert(0) += 1;

        }
        ans
    }
}