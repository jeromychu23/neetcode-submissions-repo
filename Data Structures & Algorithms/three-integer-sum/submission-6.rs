impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for (i, &n) in nums.iter().enumerate() {
            if n > 0 {
                break
            }

            if i > 0 && n == nums[i - 1] {
                continue
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let val = n + nums[l] + nums[r];
                if val > 0 {
                    r -= 1
                }
                else if val < 0 {
                    l += 1
                }
                else {
                    res.push([n, nums[l], nums[r]].to_vec());
                    l += 1;
                    r -= 1;
                    while nums[l] == nums[l - 1] && l < r {
                        l += 1
                    }
                }
            }
        }
        res
    }
}
