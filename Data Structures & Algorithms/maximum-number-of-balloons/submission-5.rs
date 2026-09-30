impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let (mut b, mut a, mut l, mut o, mut n) = (0, 0, 0, 0, 0);

        for c in text.bytes() {
            match c {
                b'b' => b += 1,
                b'a' => a += 1,
                b'l' => l += 1,
                b'o' => o += 1,
                b'n' => n += 1,
                _ => {}
            }
        }

        b.min(a).min(l / 2).min(o / 2).min(n)
    }
}