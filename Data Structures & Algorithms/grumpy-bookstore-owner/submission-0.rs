impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut count = 0;
        let mut max_win = 0;

        for (c, g) in customers.clone().into_iter().zip(grumpy.clone()) {
            if g == 0 {
                count += c;
            }
        }    

        let mut j = 0;
        let mut sum = 0;
        let m = minutes as usize;
        for (i, &g) in grumpy.iter().enumerate() {
            if g == 1 {
                sum += customers[i];
            }
            
            if i - j + 1 == m {
                max_win = max_win.max(sum);
                if grumpy[j] == 1 {
                    sum -= customers[j];
                }
                j += 1;
            }
        }
        count + max_win
    }
}