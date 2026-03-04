use crate::solution::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut p, mut q, m, n) = (0, 0, s.len(), t.len());
        let (s_chars, t_chars) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
        while p < m && q < n {
            if s_chars[p] == t_chars[q] {
                p += 1;
            }

            q += 1;
        }

        p == m
    }
}

#[test]
fn test() {
    struct TestCase {
        s: String,
        t: String,
        expected: bool,
    }

    let test_cases = [
        TestCase {
            s: "abc".to_string(),
            t: "ahbgdc".to_string(),
            expected: true,
        },
        TestCase {
            s: "axc".to_string(),
            t: "ahbgdc".to_string(),
            expected: false,
        },
    ];

    for tc in test_cases {
        assert_eq!(Solution::is_subsequence(tc.s, tc.t), tc.expected);
    }
}
