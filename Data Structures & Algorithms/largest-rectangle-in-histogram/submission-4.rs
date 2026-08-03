impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut max_area = 0;
        heights.push(0);

        for (i, cur_height) in heights.iter().enumerate() {
            let mut cur_start = i;

            while let Some(&(start, prev_height)) = stack.last() {
                if prev_height <= cur_height {
                    break
                }
                stack.pop();

                let width = i - start;
                let area = prev_height * width as i32;
                max_area = max_area.max(area);
                cur_start = start;
            }
            stack.push((cur_start, cur_height))
        }
        max_area
    }
}
