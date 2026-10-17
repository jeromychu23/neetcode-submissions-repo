impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut win_sum = 0;
        let mut i = 0;
        let mut res = 0;
        let k = k as usize;

        while i < arr.len() {
            win_sum += arr[i];
            if i + 1 >= k {
                if win_sum / k as i32 >= threshold {
                    res += 1;
                }
                win_sum -= arr[i - k + 1];
            }  
            i += 1;
        }

        res
    }
}
