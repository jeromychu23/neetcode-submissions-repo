impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let peak = (0..height.len()).max_by_key(|&i| height[i]).unwrap();
        let mut water = 0;
        let mut left_m = 0;
        for i in 0..peak {
            left_m = max(height[i], left_m);
            water += left_m - height[i];
        }

        let mut right_m = 0;
        for i in (peak+1..height.len()).rev() {
            right_m = max(height[i], right_m);
            water += right_m - height[i];
        }
        water
    }
}
