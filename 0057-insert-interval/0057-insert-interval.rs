impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort_by_key(|k| {
            k[0]
        });

        let mut ans = vec![];

        let mut cur_s = intervals[0][0];
        let mut cur_e = intervals[0][1];

        for i in 1..intervals.len() {
            let s = intervals[i][0];
            let e = intervals[i][1];

            println!("{} {}", s, e);

            if cur_e < s {
                ans.push(vec![cur_s, cur_e]);
                cur_s = s;
                cur_e = e;
                continue;
            }
            if s <= cur_e {
                cur_e = std::cmp::max(e, cur_e);
            }
        }

        ans.push(vec![cur_s, cur_e]);

        ans
    }
}