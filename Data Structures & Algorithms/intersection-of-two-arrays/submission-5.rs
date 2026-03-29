impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut seen: HashSet<i32> = nums1.into_iter().collect();
        let mut res = Vec::new();
        for num in nums2 {
            if seen.remove(&num) {
                res.push(num);
            }
        }
        res
    }
}
