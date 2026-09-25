impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                res = num
            }

            if num == res {
                count += 1
            } else {
                count -= 1
            }
        }
        res
    }
}
