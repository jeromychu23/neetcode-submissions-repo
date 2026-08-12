impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res = 0i32;

        while x != 0 {
            let Some(next) = res.checked_mul(10).and_then(|n| n.checked_add(x % 10)) else {
                return 0;
            };

            res = next;
            x /= 10
        }

        res
    }
}
