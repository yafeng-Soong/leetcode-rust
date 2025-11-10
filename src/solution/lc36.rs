use crate::solution::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![false; 9]; 9];
        let mut cols = vec![vec![false; 9]; 9];
        let mut blocks = vec![vec![false; 9]; 9];
        for i in 0..9 {
            for (j, c) in board[i].iter().enumerate() {
                if *c == '.' {
                    continue;
                }

                let num = (*c as u8 - b'1') as usize;
                let k = i / 3 * 3 + j / 3;
                if rows[i][num] || cols[j][num] || blocks[k][num] {
                    return false;
                }

                (rows[i][num], cols[j][num], blocks[k][num]) = (true, true, true);
            }
        }

        true
    }
}

#[test]
fn test() {
    struct Test {
        board: Vec<Vec<char>>,
        expected: bool,
    }

    let tests = vec![
        Test {
            board: vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            expected: true,
        },
        Test {
            board: vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            expected: false,
        },
    ];

    for t in tests {
        assert_eq!(Solution::is_valid_sudoku(t.board), t.expected);
    }
}
