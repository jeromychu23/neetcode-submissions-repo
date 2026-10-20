impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut res = 100001;

        for win in nums.windows(k as usize) {
            let high = win.last().unwrap();
            let low = win[0];
            res = res.min(high - low);
        }
        res
    }
}
