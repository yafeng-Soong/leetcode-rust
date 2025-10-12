use crate::solution::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut right: usize;
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

#[test]
fn test() {
    struct Test {
        s: String,
        expected: i32,
    }

    let tests = vec![
        Test {
            s: "Hello World".to_string(),
            expected: 5,
        },
        Test {
            s: "   fly me   to   the moon  ".to_string(),
            expected: 4,
        },
        Test {
            s: "luffy is still joyboy".to_string(),
            expected: 6,
        },
    ];

    for t in tests {
        assert_eq!(Solution::length_of_last_word(t.s), t.expected);
    }
}
