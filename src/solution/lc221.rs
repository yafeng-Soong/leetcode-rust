#![allow(clippy::needless_range_loop)]
use crate::solution::Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (mut res, m, n) = (0, matrix.len(), matrix[0].len());
        let (mut dp, mut old) = (vec![0; n + 1], vec![0; n + 1]);
        (1..=n).for_each(|j| {
            if matrix[0][j - 1] == '1' {
                (old[j], res) = (1, 1);
            }
        });

        for i in 1..m {
            for j in 1..=n {
                if matrix[i][j - 1] == '1' {
                    dp[j] = dp[j - 1].min(old[j].min(old[j - 1])) + 1;
                    res = res.max(dp[j]);
                } else {
                    dp[j] = 0;
                };
            }

            (old, dp) = (dp, old);
        }

        res * res
    }
}

#[test]
fn test() {
    struct Test {
        matrix: Vec<Vec<char>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            matrix: vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ],
            expected: 4,
        },
        Test {
            matrix: vec![vec!['0', '1'], vec!['1', '0']],
            expected: 1,
        },
        Test {
            matrix: vec![vec!['0']],
            expected: 0,
        },
        Test {
            matrix: vec![vec!['0'], vec!['1']],
            expected: 1,
        },
        Test {
            matrix: vec![vec!['1']],
            expected: 1,
        },
        Test {
            matrix: vec![
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['0', '0', '0', '0', '0'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
            ],
            expected: 4,
        },
        // Test {
        //     matrix: vec![vec!['1'; 300]; 300],
        //     expected: 90000,
        // },
    ];

    for t in tests {
        assert_eq!(Solution::maximal_square(t.matrix), t.expected);
    }
}
