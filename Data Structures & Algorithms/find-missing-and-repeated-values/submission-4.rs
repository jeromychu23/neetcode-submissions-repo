impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let mut nums_range: HashSet<i32> = (1..=(n * n)).collect();
        let mut repeat = 0;

        for num in grid.iter().flatten() {
            if !nums_range.remove(num) {
                repeat = *num;
            }
        }

        vec![repeat, *nums_range.iter().next().unwrap()]
    }
}
