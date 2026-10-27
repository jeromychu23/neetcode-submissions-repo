impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let m = minutes as usize;
        let mut l = 0;
        let (mut satisfied, mut cur_win, mut best_win) = (0, 0, 0);

        for r in 0..customers.len() {
            if grumpy[r] == 0 {
                satisfied += customers[r];
            } else {
                cur_win += customers[r];
            }

            if r - l + 1 == m {
                best_win = best_win.max(cur_win);
                if grumpy[l] == 1 {
                    cur_win -= customers[l];
                }
                l += 1;
            }
        }

        satisfied + best_win
    }
}
