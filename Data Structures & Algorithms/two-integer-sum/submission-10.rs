use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(ans) = mp.get(&diff) {
                return vec![*ans as i32, i as i32];
            };
            mp.insert(*num, i);
        };
        vec![]
    }
}
