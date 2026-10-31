impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut res = Vec::new();
        let mut num = column_number;

        while num > 0 {
            num -= 1;
            let offset = (num % 26) as u8;
            res.push((b'A' + offset) as char);
            num /= 26;
        }
        res.iter().rev().collect()
    }
}
