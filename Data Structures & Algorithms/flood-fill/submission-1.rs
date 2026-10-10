impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let orig_color = image[sr as usize][sc as usize];

        if orig_color == color {
            return image;
        }

        fn dfs(image: &mut Vec<Vec<i32>>, r: i32, c: i32, orig_color: i32, color: i32) {
            if r < 0
                || r >= image.len() as i32
                || c < 0
                || c >= image[0].len() as i32
                || image[r as usize][c as usize] != orig_color
            {
                return;
            }

            image[r as usize][c as usize] = color;

            dfs(image, r + 1, c, orig_color, color);
            dfs(image, r - 1, c, orig_color, color);
            dfs(image, r, c + 1, orig_color, color);
            dfs(image, r, c - 1, orig_color, color);
        }

        dfs(&mut image, sr, sc, orig_color, color);
        image
    }
}
