impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len() as i32;
        let mut res = Vec::new();
        let mut nums_range: HashSet<i32> = (1..=(n * n)).collect();

        for nums in &grid {
            for n in nums {
                if !nums_range.remove(n) {
                    res.push(*n)
                }
            }
        }
        for num in nums_range {
            res.push(num)
        }

        res
    }
}
