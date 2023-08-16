impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let mut ans = 1;
        let MOD = 10_i64.pow(9)+7;
        let mut base = 20;

        let mut nn = n / 2;
        let mut i = 1;

        while nn > 0 {
            if nn % 2 == 1 {
                ans = (ans * base) % MOD;
            }
            nn >>= 1;
            base = (base * base) % MOD;
        }

        if n % 2 == 1 {
            ((ans * 5) % MOD) as i32
        } else {
            ans as i32
        }
    }
}