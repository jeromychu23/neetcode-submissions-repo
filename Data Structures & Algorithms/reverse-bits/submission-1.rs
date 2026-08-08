impl Solution {
    pub fn reverse_bits(mut n: u32) -> u32 {
        let mut res = 0;
        for _ in 0..32 {
            let bit = n & 1;
            res = (res << 1) | bit;
            n >>= 1
        }
        res
    }
}
