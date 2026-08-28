impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::new();
        let mut new_n = n;

        while new_n != 1 {
            let mut sum = 0;
            while new_n > 0 {
                sum += (new_n % 10).pow(2);
                new_n /= 10
            }
            if seen.contains(&sum) {
                return false
            } else {
                seen.insert(sum);
                new_n = sum
            }
        }
        true
    }
}
