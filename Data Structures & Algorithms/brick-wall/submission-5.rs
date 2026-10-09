impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut borders = HashMap::new();
        let wall_height = wall.len() as i32;

        for w in wall {
            // 因為leetcode的test case用i32會overflow
            let mut cur_border = 0i64;

            for &n in w.iter().take(w.len() - 1) {
                cur_border += n as i64;
                *borders.entry(cur_border).or_insert(0) += 1;
            }
        }

        wall_height - borders.values().max().unwrap_or(&0)
    }
}
