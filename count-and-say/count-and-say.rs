impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_owned();
        }

        let count = Solution::count_and_say(n-1);
        let mut sep: Vec<(i32, char)> = vec![];
        let mut now: char = 'a';
        let mut now_cnt = 0;

        for c in count.chars() {
            if now != c {
                sep.push((now_cnt, now));
                now = c;
                now_cnt = 1;
            } else {
                now_cnt += 1;
            }
        }
        sep.push((now_cnt, now));

        let mut say = String::new();
        for &(cnt, c) in sep[1..].into_iter() {
            say.push_str(cnt.to_string().as_str());
            say.push(c);
        }

        say
    }
}