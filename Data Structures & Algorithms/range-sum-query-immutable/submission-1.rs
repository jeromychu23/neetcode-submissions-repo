struct NumArray {
    prefix_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum = vec![0; nums.len()];
        let mut cur_sum = 0;
        for (i, &n) in nums.iter().enumerate() {
            cur_sum += n;
            prefix_sum[i] = cur_sum;
        }
        Self { prefix_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;

        if left == 0 {
            self.prefix_sum[right]
        } else {
            self.prefix_sum[right] - self.prefix_sum[left - 1]
        }

    }
}
