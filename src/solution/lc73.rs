#![allow(clippy::ptr_arg)]
use crate::solution::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut first_row_zero, mut first_col_zero) = (false, false);
        matrix[0]
            .iter()
            .for_each(|&x| first_row_zero = first_row_zero || x == 0);
        matrix
            .iter()
            .for_each(|x| first_col_zero = first_col_zero || x[0] == 0);
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        matrix.iter_mut().skip(1).for_each(|x| {
            if x[0] == 0 {
                x.iter_mut().for_each(|x| *x = 0);
            }
        });

        for j in 1..n {
            if matrix[0][j] == 0 {
                matrix.iter_mut().for_each(|x| x[j] = 0);
            }
        }

        if first_row_zero {
            matrix[0].iter_mut().for_each(|x| *x = 0);
        }

        if first_col_zero {
            matrix.iter_mut().for_each(|x| x[0] = 0);
        }
    }
}

#[test]
fn test() {
    struct Test {
        matrix: Vec<Vec<i32>>,
        expected: Vec<Vec<i32>>,
    }

    let tests = vec![
        Test {
            matrix: vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            expected: vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
        },
        Test {
            matrix: vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
            expected: vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        },
        Test {
            matrix: vec![
                vec![1, 2, 3, 4],
                vec![5, 0, 7, 8],
                vec![0, 10, 11, 12],
                vec![13, 14, 15, 0],
            ],
            expected: vec![
                vec![0, 0, 3, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
        },
        Test {
            matrix: vec![vec![1, 0]],
            expected: vec![vec![0, 0]],
        },
        Test {
            matrix: vec![vec![1, 0, 3]],
            expected: vec![vec![0, 0, 0]],
        },
    ];

    for mut t in tests {
        Solution::set_zeroes(&mut t.matrix);
        assert_eq!(t.matrix, t.expected);
    }
}
