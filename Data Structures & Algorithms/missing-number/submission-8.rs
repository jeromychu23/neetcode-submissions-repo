impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() + 1;
        let orig: i32 = nums.iter().sum();
        let mut expect = 0i32;
        for i in 0..n {
            expect += i as i32
        }
        return expect - orig
    }
}
