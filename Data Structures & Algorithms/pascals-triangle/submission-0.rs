impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]; num_rows as usize];

        for i in 0..num_rows as usize {
            for _ in 0..=i {
                res[i].push(1_i32);
            }
            if i > 1 {
                for k in 1..i {
                    res[i][k] = res[i - 1][k - 1] + res[i - 1][k];
                }
            }
        }

        res
    }
}
