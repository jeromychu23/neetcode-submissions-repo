impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut current_max = nums[0];
        let mut current_min = nums[0];
        let mut answer = nums[0];

        for &num in nums.iter().skip(1) {
            if num < 0 {
                std::mem::swap(&mut current_max, &mut current_min);
            }

            current_max = num.max(current_max * num);
            current_min = num.min(current_min * num);

            answer = answer.max(current_max);
        }

        answer
    }
}