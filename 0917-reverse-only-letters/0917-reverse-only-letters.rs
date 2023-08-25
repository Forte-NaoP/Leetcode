impl Solution {
pub fn reverse_only_letters(s: String) -> String {

    let mut s = s.chars().collect::<Vec<char>>();
    let mut st = 0;
    let mut ed = s.len() - 1;

    while st < ed {
        if s[st].is_alphabetic() && s[ed].is_alphabetic() {
            s.swap(st, ed);
            st += 1;
            ed -= 1;
        } else {
            if !s[st].is_alphabetic() {
                st += 1;
            }
            if !s[ed].is_alphabetic() {
                ed -= 1;
            }
        }
    }

    s.iter().collect()

}
}