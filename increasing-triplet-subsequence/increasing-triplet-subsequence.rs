impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut lis = vec![nums[0]];

        let mut idx = 0;

        for &num in nums.iter() {
            if lis[idx] < num {
                lis.push(num);
                idx += 1;
            } else {
                let pos = lower_bound(&lis, num);
                lis[pos] = num;
            }
        }

        if lis.len() >= 3 {
            true
        } else {
            false
        }
    }

}

fn lower_bound(nums: &Vec<i32>, k: i32) -> usize {
    let mut lo = -1;
    let mut hi = nums.len() as i32;

    while (lo + 1) < hi {
        let mid = (lo + hi) / 2;

        if nums[mid as usize] < k {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi as usize
}