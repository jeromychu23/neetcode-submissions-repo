impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut one, mut two) = (1, 1);

        for _ in 0..n-1 {
            let temp = one;
            one = one + two;
            two = temp;
        }
        one
    }
}
