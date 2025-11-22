use crate::solution::Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (l, mut res) = (m.min(n), 0);
        let mut dp = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    dp[i][j] = true;
                    res = 1;
                }
            }
        }

        for k in 2..=l {
            let mut changed = false;
            for i in 0..=m - k {
                for j in 0..=n - k {
                    dp[i][j] = dp[i][j] && dp[i + 1][j] && dp[i][j + 1] && dp[i + 1][j + 1];
                    if dp[i][j] {
                        res = k;
                        changed = true;
                    }
                }
            }

            if !changed {
                return (res * res) as i32;
            }
        }

        (res * res) as i32
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
            matrix: vec![vec!['1'; 300]; 300],
            expected: 90000,
        },
    ];

    for t in tests {
        assert_eq!(Solution::maximal_square(t.matrix), t.expected);
    }
}
