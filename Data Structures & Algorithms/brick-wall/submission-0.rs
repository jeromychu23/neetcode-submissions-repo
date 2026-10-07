impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut borders = HashMap::new();
        let wall_height = wall.len() as i32;

        for w in wall {
            let mut cur_border = 0;
            let wall_len = w.len() - 1;
            for n in 0..wall_len {
                cur_border += w[n];
                *borders.entry(cur_border).or_insert(0) += 1;
            }
        }
        println!("{:?}", borders);

        if let Some(n) = borders.values().max() {
            return wall_height - n;
        }

        wall_height
    }
}
