impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut i = 0;
        let len = flowerbed.len();

        while i < len && n > 0 {
            if flowerbed[i] == 1 {
                i += 2;
            } else if i + 1 == len || flowerbed[i + 1] == 0 {
                n -= 1;
                i += 2;
            } else {
                i += 3;
            }
        }
        n <= 0
    }
}
