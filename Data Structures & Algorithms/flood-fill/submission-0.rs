impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut visited = HashSet::new();
        let (rows, cols) = (image.len() - 1, image[0].len() - 1);

        fn dfs(
            image: &Vec<Vec<i32>>,
            rows: usize,
            cols: usize,
            visited: &mut HashSet<(i32, i32)>,
            sr: i32,
            sc: i32,
            cur_color: i32,
        ) {
            if sr < 0
                || sr > rows as i32
                || sc < 0
                || sc > cols as i32
                || visited.contains(&(sr, sc))
            {
                return;
            }
            let color = image[sr as usize][sc as usize];
            if color != cur_color {
                return;
            }

            visited.insert((sr, sc));

            dfs(image, rows, cols, visited, sr + 1, sc, color);
            dfs(image, rows, cols, visited, sr - 1, sc, color);
            dfs(image, rows, cols, visited, sr, sc + 1, color);
            dfs(image, rows, cols, visited, sr, sc - 1, color);
        }

        dfs(
            &image,
            rows,
            cols,
            &mut visited,
            sr,
            sc,
            image[sr as usize][sc as usize],
        );
        println!("{:?}", visited);

        for (i, j) in visited {
            image[i as usize][j as usize] = color
        }

        image
    }
}
