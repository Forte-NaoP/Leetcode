impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; 200002];
        let offset = 100000_usize;

        for i in nums.iter() {
            count[*i as usize + offset] += 1;
        }

        let mut cur = 0;
        let mut ans = 0;
        for i in (0..=200001).rev() {
            cur += count[i];
            if cur >= k {
                ans = i;
                break;
            }
        }
        (ans - offset) as i32
    }
}