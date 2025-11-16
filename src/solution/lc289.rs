#![allow(clippy::ptr_arg)]
use crate::solution::Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let dir = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let (m, n) = (board.len(), board[0].len());
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                for d in dir {
                    let (x, y) = (i as i32 + d.0, j as i32 + d.1);
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }

                    match board[x as usize][y as usize] {
                        1 | 3 | 4 => sum += 1,
                        _ => (),
                    }
                }

                match (board[i][j], sum) {
                    (0, 3) => board[i][j] = 2,
                    (1, 2) | (1, 3) => board[i][j] = 3,
                    (1, _) => board[i][j] = 4,
                    _ => (),
                }
            }
        }

        board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|x| match x {
                1..=3 => *x = 1,
                _ => *x = 0,
            });
        });
    }
}

#[test]
fn test() {
    struct Test {
        board: Vec<Vec<i32>>,
        expected: Vec<Vec<i32>>,
    }

    let tests = vec![
        Test {
            board: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
            expected: vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
        },
        Test {
            board: vec![vec![1, 1], vec![1, 0]],
            expected: vec![vec![1, 1], vec![1, 1]],
        },
    ];

    for mut t in tests {
        Solution::game_of_life(&mut t.board);
        assert_eq!(t.board, t.expected);
    }
}
