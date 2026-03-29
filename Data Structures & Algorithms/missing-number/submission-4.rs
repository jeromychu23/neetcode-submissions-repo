impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let orig: i32 = nums.iter().sum();
        let mut expect = 0i32;
        for i in 0..n+1 {
            expect += i
        }
        expect - orig
    }
}
