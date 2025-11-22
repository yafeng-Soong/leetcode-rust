use crate::solution::Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        if m == 0 || n == 0 {
            return m.max(n) as i32;
        }

        let (chars1, chars2) = (word1.into_bytes(), word2.into_bytes());
        let (mut dp, mut old) = (vec![0; n + 1], vec![0; n + 1]);
        old.iter_mut().enumerate().for_each(|(i, x)| *x = i);

        for i in 1..=m {
            dp[0] = i;
            for j in 1..=n {
                let tmp = if chars1[i - 1] == chars2[j - 1] {
                    old[j - 1]
                } else {
                    old[j - 1] + 1
                };

                dp[j] = tmp.min(old[j].min(dp[j - 1]) + 1);
            }

            (old, dp) = (dp, old);
        }

        old[n] as i32
    }
}

#[test]
fn test() {
    struct Test {
        word1: String,
        word2: String,
        expected: i32,
    }

    let tests = vec![
        Test {
            word1: "horse".to_string(),
            word2: "ros".to_string(),
            expected: 3,
        },
        Test {
            word1: "intention".to_string(),
            word2: "execution".to_string(),
            expected: 5,
        },
        Test {
            word1: "".to_string(),
            word2: "".to_string(),
            expected: 0,
        },
        Test {
            word1: "a".to_string(),
            word2: "ab".to_string(),
            expected: 1,
        },
        Test {
            word1: "".to_string(),
            word2: "a".to_string(),
            expected: 1,
        },
        Test {
            word1: "sea".to_string(),
            word2: "eat".to_string(),
            expected: 2,
        },
    ];

    for t in tests {
        assert_eq!(
            Solution::min_distance(t.word1.clone(), t.word2.clone()),
            t.expected
        );
    }
}
