impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut minBuy = &prices[0];
        for p in &prices {
            minBuy = min(&p, minBuy);
            profit = max(p - minBuy, profit);
        }
        profit
    }
}
