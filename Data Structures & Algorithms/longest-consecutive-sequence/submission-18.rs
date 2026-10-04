impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<_> = nums.iter().clone().collect();
        let mut max_len = 0;

        for &num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let mut cur_len = 1;

                while num_set.contains(&(num + cur_len)) {
                    cur_len += 1;
                }

                max_len = max_len.max(cur_len);
            }
        }

        max_len
    }
}
