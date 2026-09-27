impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut count = HashMap::new();
        let mut max_odd = 0;
        let mut min_even = 100;

        for c in s.as_bytes() {
            *count.entry(c).or_insert(0) += 1;
        }

        for n in count {
            let freq = n.1;

            if freq % 2 == 1 && freq > max_odd {
                max_odd = freq;
            } else if freq % 2 == 0 && freq < min_even {
                min_even = freq;
            }
        }

        max_odd - min_even
    }
}
