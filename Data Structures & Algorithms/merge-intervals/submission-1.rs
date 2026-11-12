impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut stack: Vec<Vec<i32>> = Vec::new();
        intervals.sort_by_key(|x| x[0]);

        for interval in intervals {
            let mut cur = interval;
            if let Some(top) = stack.pop() {
                if cur[0] > top[1] {
                    res.push(top);
                } else if cur[0] <= top[1] && cur[1] <= top[1] {
                    cur = top;
                } else {
                    cur[0] = top[0];
                }
            }
            stack.push(cur);
        } 
        res.append(&mut stack);
        res
    }
}
