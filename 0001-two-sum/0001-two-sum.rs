impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut nums_idxed = vec![];

        for (idx, val) in nums.iter().enumerate() {
            nums_idxed.push((*val, idx as i32));
        }

        nums_idxed.sort();

        //println!("{:?}", nums_idxed);

        let len = nums_idxed.len();
        let mut sum: i32 = 0;
        let mut diff: i32 = 0;
        let (mut left, mut right, mut mid): (usize, usize, usize);
        let mut ans: Vec<i32> = vec![];

        for i in 0..len {
            sum = nums_idxed[i].0;
            diff = target - sum;
            left = i + 1;
            right = len - 1;

            //println!("now: sum {}, diff {}, i {} left {} right {}", sum, diff, i, left, right);

            while left <= right {
                mid = (left + right) / 2;
                if nums_idxed[mid].0 > diff {
                    right = mid - 1;
                } else if nums_idxed[mid].0 < diff {
                    left = mid + 1;
                } else {
                    //println!("{}, {}, {}, {}", sum, diff, i, mid);
                    ans.push(nums_idxed[i].1);
                    ans.push(nums_idxed[mid].1);
                    break;
                }
            }

            if ans.len() != 0 {
                break;
            }
        }

        ans
    }

}