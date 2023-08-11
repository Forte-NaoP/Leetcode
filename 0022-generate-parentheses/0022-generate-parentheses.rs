impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut s = vec![];

        let mut st = vec![];
        let mut paren = "".to_owned();
        _generate_parenthesis(paren, st, n, &mut s);

        s
    }

}

fn _generate_parenthesis(paren: String, st: Vec<char>, n: i32, ans: &mut Vec<String>){
    let mut paren = paren;
    let mut st = st;

    if n == 0 {
        while !st.is_empty() {
            paren.push(*st.last().unwrap());
            st.pop();
        }
        //println!("{}", paren);
        ans.push(paren);
        return;
    }

    if st.is_empty() {
        let mut st_c = st.clone();
        st_c.push(')');
        _generate_parenthesis(format!("{}(", paren).to_owned(), st_c, n-1, ans);
    } else {
        let mut st_c = st.clone();
        st_c.push(')');
        _generate_parenthesis(format!("{}(", paren).to_owned(), st_c, n-1, ans);

        paren.push(*st.last().unwrap());
        st.pop();
        _generate_parenthesis(paren.to_owned(), st, n, ans);
    }
}