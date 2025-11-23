use crate::solution::Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m + n != s3.len() {
            return false;
        }

        let (chars1, chars2, chars3) = (s1.into_bytes(), s2.into_bytes(), s3.into_bytes());
        let (mut dp, mut prev) = (vec![false; n + 1], vec![false; n + 1]);
        prev[0] = true;
        for i in 1..=n {
            if chars2[i - 1] == chars3[i - 1] {
                prev[i] = true;
            } else {
                break;
            }
        }

        for i in 1..=m {
            dp[0] = if chars1[i - 1] == chars3[i - 1] {
                prev[0]
            } else {
                false
            };

            for j in 1..=n {
                dp[j] = (prev[j] && chars1[i - 1] == chars3[i + j - 1])
                    || (dp[j - 1] && chars2[j - 1] == chars3[i + j - 1])
            }

            (prev, dp) = (dp, prev);
        }

        prev[n]
    }
}

#[test]
fn test() {
    struct Test {
        s1: String,
        s2: String,
        s3: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            s1: "aabcc".to_string(),
            s2: "dbbca".to_string(),
            s3: "aadbbcbcac".to_string(),
            expected: true,
        },
        Test {
            s1: "aabcc".to_string(),
            s2: "dbbca".to_string(),
            s3: "aadbbbaccc".to_string(),
            expected: false,
        },
        Test {
            s1: "".to_string(),
            s2: "".to_string(),
            s3: "".to_string(),
            expected: true,
        },
        Test {
            s1: "a".to_string(),
            s2: "b".to_string(),
            s3: "ab".to_string(),
            expected: true,
        },
        Test {
            s1: "a".to_string(),
            s2: "b".to_string(),
            s3: "ba".to_string(),
            expected: true,
        },
        Test {
            s1: "a".to_string(),
            s2: "b".to_string(),
            s3: "ac".to_string(),
            expected: false,
        },
        Test {
            s1: "".to_string(),
            s2: "".to_string(),
            s3: "a".to_string(),
            expected: false,
        },
    ];

    for t in tests {
        assert_eq!(Solution::is_interleave(t.s1, t.s2, t.s3), t.expected);
    }
}
