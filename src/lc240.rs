use crate::solution::Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut x, mut y) = (0, n as i32 - 1);
        while x < m && y >= 0 {
            match matrix[x][y as usize].cmp(&target) {
                Ordering::Less => x += 1,
                Ordering::Equal => return true,
                Ordering::Greater => y -= 1,
            }
        }

        false
    }
}

#[test]
fn test() {
    struct Test {
        matrix: Vec<Vec<i32>>,
        target: i32,
        expected: bool,
    }

    let tests = vec![
        Test {
            matrix: vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            target: 5,
            expected: true,
        },
        Test {
            matrix: vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            target: 20,
            expected: false,
        },
    ];

    for t in tests {
        assert_eq!(Solution::search_matrix(t.matrix, t.target), t.expected);
    }
}
