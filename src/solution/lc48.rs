use crate::solution::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
            }
        }

        for row in matrix {
            let (mut left, mut right) = (0, n - 1);
            while left < right {
                row.swap(left, right);
                left += 1;
                right -= 1;
            }
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
            matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            expected: vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
        },
        Test {
            matrix: vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ],
            expected: vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ],
        },
    ];

    for mut t in tests {
        Solution::rotate(&mut t.matrix);
        assert_eq!(t.matrix, t.expected);
    }
}
