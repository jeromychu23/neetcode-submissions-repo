impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let target = amount as usize;
        let unreachable = target + 1;

        let coins: Vec<usize> = coins
            .into_iter()
            .map(|coin| coin as usize)
            .collect();

        let mut dp = vec![unreachable; target + 1];
        dp[0] = 0;

        for current in 1..=target {
            for &coin in &coins {
                if coin <= current {
                    dp[current] = dp[current].min(dp[current - coin] + 1);
                }
            }
        }
        if dp[target] == unreachable {
            -1
        } else {
            dp[target] as i32
        }
    }
}