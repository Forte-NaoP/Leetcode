impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

        if s.len() == 0 {
            return 0
        }

        let mut ans = 0;
        let mut indices = vec![-1; 128];
        let (mut st, mut ed) = (0, 0);

        for (idx, c) in s.chars().enumerate().map(|(i, c)| (i as i32, c as usize)) {
            if indices[c] == -1 || !(st..ed).contains(&indices[c]) {
                indices[c] = idx;
                ed = idx+1;
            } else {
                if indices[c] == st {
                    st += 1;
                } else {
                    ans = std::cmp::max(ans, ed-st);
                    st = indices[c] + 1;
                }
                ed = idx+1;
                indices[c] = idx;
            }
        }
        ans = std::cmp::max(ans, ed-st);
        ans
    }
}
