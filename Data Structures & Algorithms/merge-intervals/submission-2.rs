impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        intervals.sort_by_key(|x| x[0]);

        for cur in intervals {
            if let Some(last) = res.last_mut() {
                if cur[0] <= last[1] {
                    last[1] = last[1].max(cur[1]);
                    continue;
                } 
            }
            res.push(cur);
        } 
        res
    }
}
