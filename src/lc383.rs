use crate::solution::Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnts = [0; 26];
        for c in magazine.chars() {
            cnts[c as usize - 'a' as usize] += 1;
        }

        for c in ransom_note.chars() {
            cnts[c as usize - 'a' as usize] -= 1;
            if cnts[c as usize - 'a' as usize] < 0 {
                return false;
            }
        }

        true
    }
}
