impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut res = Vec::new();

        while i < first_list.len() && j < second_list.len() {
            let start = first_list[i][0].max(second_list[j][0]);
            let end = first_list[i][1].min(second_list[j][1]);

            if start <= end {
                res.push(vec![start, end]);
            }

            if end == first_list[i][1] {
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }
}
