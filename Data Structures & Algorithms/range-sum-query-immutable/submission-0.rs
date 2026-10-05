struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut res = 0;

        for i in (left..=right) {
            res += self.nums[i as usize];
        }

        res
    }
}
