impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cur = Vec::new();

        fn backtrack(start: i32, n: i32, k: i32, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if cur.len() == k as usize {
                res.push(cur.clone());
                return;
            }

            for i in start..=n {
                cur.push(i);
                backtrack(i + 1, n, k, cur, res);
                cur.pop();
            }
        }
        backtrack(1, n, k, &mut cur, &mut res);
        res
    }
}
