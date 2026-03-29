impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();
        let mut col: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();
        let mut square: Vec<HashSet<char>> = (0..9).map(|_| HashSet::new()).collect();

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];
                if val == '.' {
                    continue
                }
                let box_idx = (r / 3) * 3 + c / 3;
                if !row[r].insert(val) || !col[c].insert(val) || !square[box_idx].insert(val) {
                    return false;
                }
            }
        }
        true
    }
}
