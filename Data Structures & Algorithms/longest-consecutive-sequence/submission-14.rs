impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut mp: HashSet<_> = nums.iter().copied().collect();
        let mut longest = 0;
        for &num in &mp {
            if !mp.contains(&(num - 1)) {
                let mut length = 1;
                while mp.contains(&(num + length)) {
                    length += 1;
                }
                longest = max(length, longest);
            }
        }
        longest
    }
}
