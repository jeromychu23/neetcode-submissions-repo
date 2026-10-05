impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len() - 1, grid[0].len() - 1);
        let mut res = 0;

        for i in 0..=row {
            for j in 0..=col {
                if grid[i][j] == 1 {
                    res += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        res -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        res -= 2;
                    }
                }
            }
        }
        res
    }
}
