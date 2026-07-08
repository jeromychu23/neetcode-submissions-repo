impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let n = cost.len();
        if let Some(last) = n.checked_sub(3) {
            for i in (0..=last).rev() {
                cost[i] += cost[i + 1].min(cost[i + 2]);
            }
        }
        cost[0].min(cost[1])
    }
}
