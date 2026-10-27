impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut res = students.len() as i32;
        let mut count = [0i32; 2];
    
        for s in students {
            count[s as usize] += 1;
        }

        for s in sandwiches {
            if count[s as usize] > 0 {
                count[s as usize] -= 1;
                res -= 1;
            } else {
                return res;
            }
        }
        res
    }
}