use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (m, n) = (s.len(), t.len());
        if m < n {
            return String::new();
        }

        let (mut left, mut right, mut cnt) = (0, 0, n);
        let (mut res_left, mut res_right) = (0, 0);
        let mut map = HashMap::new();
        for c in t.into_bytes() {
            map.entry(c).and_modify(|val| *val += 1).or_insert(1);
        }

        let chars = s.into_bytes();
        while left < m {
            while right < m && cnt > 0 {
                let c = chars[right];
                if let Some(val) = map.get_mut(&c) {
                    *val -= 1;
                    if *val >= 0 {
                        cnt -= 1;
                    }
                }

                right += 1;
            }

            if cnt != 0 {
                break;
            }

            while match map.get_mut(&chars[left]) {
                None => true,
                Some(val) if *val < 0 => {
                    *val += 1;
                    true
                }
                _ => false,
            } {
                left += 1;
            }

            if res_right == 0 || right - left < res_right - res_left {
                (res_left, res_right) = (left, right);
            }

            map.entry(chars[left]).and_modify(|x| *x += 1);
            left += 1;
            cnt += 1;
        }

        String::from_utf8(chars[res_left..res_right].to_vec()).unwrap()
    }
}

#[test]
fn test() {
    struct Test {
        s: String,
        t: String,
        expected: String,
    }

    let tests = vec![
        Test {
            s: "ADOBECODEBANC".to_string(),
            t: "ABC".to_string(),
            expected: "BANC".to_string(),
        },
        Test {
            s: "a".to_string(),
            t: "a".to_string(),
            expected: "a".to_string(),
        },
        Test {
            s: "a".to_string(),
            t: "aa".to_string(),
            expected: "".to_string(),
        },
        Test {
            s: "acbdbaab".to_string(),
            t: "aabd".to_string(),
            expected: "dbaa".to_string(),
        },
        Test {
            s: "bba".to_string(),
            t: "ab".to_string(),
            expected: "ba".to_string(),
        },
    ];

    for t in tests {
        assert_eq!(Solution::min_window(t.s, t.t), t.expected);
    }
}
