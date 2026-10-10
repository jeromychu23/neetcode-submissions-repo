impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = HashSet::new();
        let mut position = (0, 0);
        for p in path.chars() {
            visited.insert(position);

            match p {
                'N' => position.1 += 1,
                'E' => position.0 += 1,
                'S' => position.1 -= 1,
                'W' => position.0 -= 1,
                _ => continue,
            }

            if visited.contains(&position) {
                return true;
            }
        }
        false
    }
}
