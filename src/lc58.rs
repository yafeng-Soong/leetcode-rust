use crate::solution::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        let (mut left, mut right) = (0, 0);
        let n = s.len();
        let chars = s.as_bytes();
        while left < n {
            while left < n && chars[left] == b' ' {
                left += 1;
            }

            if left == n {
                break;
            }

            right = left;
            while right < n && chars[right] != b' ' {
                right += 1;
            }

            res = right - left;
            left = right;
        }

        res as i32
    }
}
