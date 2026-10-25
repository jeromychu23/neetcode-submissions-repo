impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num);
                continue;
            }

            let num1 = stack.pop().unwrap();
            let num2 = stack.pop().unwrap();
            match token.as_str() {
                "+" => stack.push(num2 + num1),
                "-" => stack.push(num2 - num1),
                "*" => stack.push(num2 * num1),
                "/" => stack.push(num2 / num1),
                _ => continue,
            }
        }
        stack[0]
    }
}
