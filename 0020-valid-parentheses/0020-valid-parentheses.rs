impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = vec![];
        let left = "({[";
        let right = ")}]";
        let mut ans = true;

        for p in s.chars() {
            match left.find(p) {
                Some(val) => {
                    st.push(right.chars().nth(val).unwrap());
                },
                None => {
                    match right.find(p) {
                        Some(_) => {
                            match st.last() {
                                Some(val) => {
                                    if *val == p {
                                        st.pop().unwrap();
                                    } else {
                                        ans = false;
                                        break;
                                    }
                                },
                                None => {
                                    ans = false;
                                    break;
                                }
                            }
                        },
                        None => ()
                    }
                }
            } 
        }

        if !st.is_empty() {
            return false;
        }

        ans

    }
}