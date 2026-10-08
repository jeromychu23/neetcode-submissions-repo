impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut mag_cnt = [0; 26];

        for byte in magazine.as_bytes() {
            let i = byte - b'a';
            mag_cnt[i as usize] += 1;
        }

        for byte in ransom_note.as_bytes() {
            let i = byte - b'a';
            if mag_cnt[i as usize] == 0 {
                return false;
            } else {
                mag_cnt[i as usize] -= 1
            }
        }
        true
    }
}
