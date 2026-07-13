impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            n => {
                let skip_first = Self::helper(&nums[1..]);
                let skip_last = Self::helper(&nums[..n-1]);
                skip_first.max(skip_last)
            },
        }
    }
    fn helper(nums: &[i32]) -> i32 {
        let (mut rob1, mut rob2) = (0i32, 0i32);
        for n in nums {
            let temp = (n + rob1).max(rob2);
            rob1 = rob2;
            rob2 = temp
        }
        rob2
    }
}
