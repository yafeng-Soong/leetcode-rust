use std::collections::HashMap;

use crate::solution::Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut cnts = HashMap::new();
        let mut left = 0;
        for (right, c) in s.chars().enumerate() {
            if let Some(idx) = cnts.get(&c) {
                left = left.max(idx + 1);
            }

            res = res.max(right - left + 1);
            cnts.insert(c, right);
        }

        res as i32
    }
}

#[test]
fn test() {
    struct Test {
        input: String,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: "".to_string(),
            expected: 0,
        },
        Test {
            input: "abcabcbb".to_string(),
            expected: 3,
        },
        Test {
            input: "bbbbb".to_string(),
            expected: 1,
        },
        Test {
            input: "pwwkew".to_string(),
            expected: 3,
        },
        Test {
            input: "tmmzuxt".to_string(),
            expected: 5,
        },
    ];

    for t in tests {
        assert_eq!(Solution::length_of_longest_substring(t.input), t.expected);
    }
}
