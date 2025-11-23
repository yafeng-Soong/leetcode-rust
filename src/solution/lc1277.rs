#![allow(clippy::needless_range_loop)]
use crate::solution::Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (mut sum, m, n) = (0, matrix.len(), matrix[0].len());
        let (mut dp, mut prev) = (vec![0; n + 1], vec![0; n + 1]);
        for i in 1..=m {
            for j in 1..=n {
                if matrix[i - 1][j - 1] == 1 {
                    dp[j] = dp[j - 1].min(prev[j].min(prev[j - 1])) + 1;
                    sum += dp[j];
                } else {
                    dp[j] = 0;
                }
            }

            (prev, dp) = (dp, prev);
        }

        sum
    }
}

#[test]
fn test() {
    struct Test {
        matrix: Vec<Vec<i32>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            matrix: vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]],
            expected: 15,
        },
        Test {
            matrix: vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]],
            expected: 7,
        },
    ];

    for t in tests {
        assert_eq!(Solution::count_squares(t.matrix), t.expected);
    }
}
