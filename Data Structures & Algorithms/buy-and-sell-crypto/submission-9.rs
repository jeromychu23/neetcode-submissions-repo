impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_buy = &prices[0];
        for p in &prices {
            min_buy = min(p, min_buy);
            profit = max(p - min_buy, profit);
        }
        profit
    }
}
