impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let k = k as usize;
        let mut l = 0;
        let mut res = 0;

        for r in 0..(n + k - 1) {
            if r > 0 && colors[r % n] == colors[(r - 1) % n] {
                l = r; 
            }
            if r - l + 1 > k {
                l += 1;
            }
            if r - l + 1 == k {
                res += 1;
            }
        }
        res
    }
}
