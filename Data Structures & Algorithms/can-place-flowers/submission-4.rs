impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let mut i = 0 as usize;
        let len = flowerbed.len();

        while i < len {
            if flowerbed[i] == 1 {
                i += 1;
                continue;
            }
            if i > 0 && i < len - 1 && flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                flowerbed[i] = 1;
                n -= 1;
                println!("{}", i);
            }
            if i == 0 && (len == 1 || flowerbed[i + 1] == 0) {
                flowerbed[i] = 1;
                n -= 1;
                println!("{}", i);
            } else if i == len - 1 && flowerbed[i - 1] == 0 {
                flowerbed[i] = 1;
                n -= 1;
                println!("{}", i);
            }
            if n == 0 {
                return true;
            }
            i += 1;
        }
        false

    }
}
