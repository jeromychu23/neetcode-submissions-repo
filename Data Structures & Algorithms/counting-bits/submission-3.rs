impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];

        for i in 1..=n as usize {
            res[i] = res[i >> 1] + (i & 1) as i32;
        }

        res
    }
}
