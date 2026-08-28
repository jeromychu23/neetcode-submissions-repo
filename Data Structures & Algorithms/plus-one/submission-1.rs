impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut num = 1u64;
        let mut p = digits.len() as u32 - 1;

        for d in digits {
            num += d as u64 * 10u64.pow(p);
            p -= 1
        }

        while num > 0 {
            res.push(num % 10);
            num /= 10;
        }
        
        res.reverse();
        res.iter().map(|&d| d as i32).collect()
    }
}
