impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mp: HashSet<_> = nums.iter().copied().collect();
        let mut longest = 0;
        for &n in &mp {
            if !mp.contains(&(n - 1)) {
                let mut length = 1;
                while mp.contains(&(n + length)) {
                    length += 1;
                }
                longest = max(length, longest);
            }
        }
        longest
    }
}
