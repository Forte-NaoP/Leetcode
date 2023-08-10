impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashSet;

        let nums = nums
            .into_iter().collect::<HashSet<i32>>()
            .into_iter().collect::<Vec<i32>>();

        let mut nums_idxed = vec![];

        for (idx, val) in nums.iter().enumerate() {
            nums_idxed.push((val.count_ones() as i32, *val, idx as i32));
        }

        nums_idxed.sort();

        //println!("{:?}", nums_idxed);

        let len = nums_idxed.len();
        let mut ans = 0_i64;

        for i in 0..len {
            let sum = nums_idxed[i].0;
            let diff = k - sum;
            
            let l_bound = lower_bound(&nums_idxed[i..], diff);

            if l_bound + i == len {
                continue;
            }
            //println!("from {} {}, diff {}, l_bound {} {:?} ", i, nums_idxed[i].1, diff, l_bound + i, nums_idxed[l_bound + i]);

            ans += ((len - (l_bound + i)) * 2) as i64;
            if l_bound == 0 {
                ans -= 1;
            }
        }

        ans
    }


}

fn lower_bound(num: &[(i32, i32, i32)], k: i32) -> usize {
    let n = num.len() as i32;
    let mut lo = -1;
    let mut hi = n;

    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if num[mid as usize].0 < k {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi as usize
}