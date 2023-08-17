impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let operator = "+-*/";
        let mut stack: Vec<i32> = vec![];

        for token in tokens.iter() {
            if let Some(_) = operator.find(token) {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match token.as_str() {
                    "+" => {
                        stack.push(left+right);
                    }, 
                    "-" => {
                        stack.push(left-right);
                    },
                    "*" => {
                        stack.push(left*right);
                    },
                    "/" => {
                        stack.push(left/right);
                    },
                    _ => ()
                }
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack.pop().unwrap()
    }
}