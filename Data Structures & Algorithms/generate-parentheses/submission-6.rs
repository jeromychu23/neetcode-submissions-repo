impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut cur: Vec<String> = vec![];

        fn backtrack(
            result: &mut Vec<String>,
            current: &mut Vec<String>,
            n: i32,
            open_count: i32,
            close_count: i32,
        ) {
            if open_count == n && close_count == n {
                result.push(current.join(""));
                return;
            }
            if open_count < n {
                current.push("(".to_string());
                backtrack(result, current, n, open_count + 1, close_count);
                current.pop();
            }
            if open_count > close_count {
                current.push(")".to_string());
                backtrack(result, current, n, open_count, close_count + 1);
                current.pop();
            }
        }
        backtrack(&mut res, &mut cur, n, 0, 0);
        res
    }
}
