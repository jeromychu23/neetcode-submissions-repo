impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..=n as usize {
            let mut cur_count = 0;
            let mut cur_n = i;
            while cur_n != 0 {
                cur_n &= cur_n - 1;
                cur_count += 1;
            }
            res.push(cur_count)
        }
        res
    }
}
