impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for &n in &nums {
            *count.entry(n).or_insert(0usize) += 1;
        }
        
        let mut bucket = vec![vec![]; nums.len() + 1];
        for (&n, &freq) in &count {
            bucket[freq].push(n);
        }

        let k = k as usize;
        let mut res = Vec::new();
        for i in (1..bucket.len()).rev() {
            for &num in &bucket[i] {
                res.push(num);
                if res.len() == k {
                    return res;
                }
            }
        }
        res
    }
}
