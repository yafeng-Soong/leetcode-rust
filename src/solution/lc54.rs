use crate::solution::Solution;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        const USED: i32 = 101;
        let dir = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut x, mut y, mut d) = (0_i32, 0_i32, 0);
        let (m, n) = (matrix.len(), matrix[0].len());
        for _ in 0..m * n {
            res.push(matrix[x as usize][y as usize]);
            matrix[x as usize][y as usize] = USED;
            let (next_x, next_y) = (x + dir[d].0, y + dir[d].1);
            if next_x < 0
                || next_x >= m as i32
                || next_y < 0
                || next_y >= n as i32
                || matrix[next_x as usize][next_y as usize] == USED
            {
                d = (d + 1) % 4;
            }

            (x, y) = (x + dir[d].0, y + dir[d].1);
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        matrix: Vec<Vec<i32>>,
        expected: Vec<i32>,
    }

    let tests = vec![
        Test {
            matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            expected: vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        },
        Test {
            matrix: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
            expected: vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        },
    ];

    for t in tests {
        assert_eq!(Solution::spiral_order(t.matrix), t.expected);
    }
}
