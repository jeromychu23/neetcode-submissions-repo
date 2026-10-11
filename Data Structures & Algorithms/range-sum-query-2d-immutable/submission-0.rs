struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut res = 0;
        for r in row1..=row2 {
            for c in col1..=col2 {
                res += self.matrix[r as usize][c as usize];
            }
        }
        res
    }
}
