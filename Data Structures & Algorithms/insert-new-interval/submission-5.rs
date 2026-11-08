impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut inserted = false;
        let mut res = Vec::new();

        for each in intervals {
            if each[0] > new_interval[1] {
                if !inserted {
                    res.push(new_interval.clone());
                    inserted = true;
                }
                res.push(each);
            } else if each[1] < new_interval[0] {
                res.push(each);
            } else {
                new_interval[0] = new_interval[0].min(each[0]);
                new_interval[1] = new_interval[1].max(each[1]);
            }
        }
        if !inserted {
            res.push(new_interval);
        }
        res
    }
}
