impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
            return -1;
        }

        let mut res = 0;
        let mut total = 0;
        for i in 0..gas.len() {
            total += gas[i] - cost[i];
            if total < 0 {
                total = 0;
                res = i as i32 + 1;
            }
        }
        res
    }
}
