use crate::solution::Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let (mut res, n) = (Vec::new(), n as usize);
        let (mut cols, mut dig1, mut dig2) =
            (vec![false; n], vec![false; 2 * n], vec![false; 2 * n]);
        let mut board = vec![vec!['.'; n]; n];
        fn backtrace(
            n: usize,
            x: usize,
            res: &mut Vec<Vec<String>>,
            board: &mut Vec<Vec<char>>,
            cols: &mut Vec<bool>,
            dig1: &mut Vec<bool>,
            dig2: &mut Vec<bool>,
        ) {
            if x == n {
                res.push(board.iter().map(|row| row.iter().collect()).collect());
                return;
            }

            for j in 0..n {
                if cols[j] || dig1[x + j] || dig2[n + x - j] {
                    continue;
                }

                (cols[j], dig1[x + j], dig2[n + x - j]) = (true, true, true);
                board[x][j] = 'Q';
                backtrace(n, x + 1, res, board, cols, dig1, dig2);
                board[x][j] = '.';
                (cols[j], dig1[x + j], dig2[n + x - j]) = (false, false, false);
            }
        }

        backtrace(n, 0, &mut res, &mut board, &mut cols, &mut dig1, &mut dig2);
        res
    }
}

#[test]
fn test() {
    struct Test {
        n: i32,
        expected: Vec<Vec<String>>,
    }

    let tests = vec![
        Test {
            n: 4,
            expected: vec![
                vec![
                    String::from(".Q.."),
                    String::from("...Q"),
                    String::from("Q..."),
                    String::from("..Q."),
                ],
                vec![
                    String::from("..Q."),
                    String::from("Q..."),
                    String::from("...Q"),
                    String::from(".Q.."),
                ],
            ],
        },
        Test {
            n: 1,
            expected: vec![vec![String::from("Q")]],
        },
    ];

    for t in tests {
        assert_eq!(Solution::solve_n_queens(t.n), t.expected);
    }
}
