use std::collections::VecDeque;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back((amount, 0));
        seen.insert(amount);

        while !q.is_empty() {
            let Some((amount, count)) = q.pop_front() else {
                break
            };
            if amount == 0 {
                return count;
            }
            for c in &coins {
                let amount_left = amount - c;
                if amount_left >= 0 && !seen.contains(&amount_left) {
                    seen.insert(amount_left);
                    q.push_back((amount_left, count + 1))
                }
            }
        }
        -1
    }
}
