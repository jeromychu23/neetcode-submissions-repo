impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|x| x[0]);

        let mut res = 0;
        let mut prev_end = intervals[0][1];
        for inter in intervals.iter().skip(1) {
            let cur_start = inter[0];
            let cur_end = inter[1];
            if cur_start < prev_end {
                res += 1;
                prev_end = prev_end.min(cur_end);
            } else {
                prev_end = cur_end;
            }
        }
        res
    }
}
