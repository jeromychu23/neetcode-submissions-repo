struct NumMatrix {
    prefix_matrix: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut prefix_matrix = vec![vec![0; col + 1]; row + 1];

        for r in 0..row {
            let mut prefix = 0;
            for c in 0..col {
                prefix += matrix[r][c];
                let above = prefix_matrix[r][c + 1];
                prefix_matrix[r + 1][c + 1] = prefix + above;
            }
        }

        Self { prefix_matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (
            row1 as usize + 1,
            col1 as usize + 1,
            row2 as usize + 1,
            col2 as usize + 1,
        );

        let above = self.prefix_matrix[row1 - 1][col2];
        let left = self.prefix_matrix[row2][col1 - 1];
        let top_left = self.prefix_matrix[row1 - 1][col1 - 1];

        self.prefix_matrix[row2][col2] - above - left + top_left
    }
}
