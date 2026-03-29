impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut water = 0;
        let (mut l, mut r) = (0, heights.len() - 1);
        while l < r {
            let curr = min(heights[l], heights[r]) * (r as i32 - l as i32);
            water = max(curr, water);
            if heights[l] < heights[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        water
    }
}
