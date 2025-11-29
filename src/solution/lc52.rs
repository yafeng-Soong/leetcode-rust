use crate::solution::Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let (mut res, n) = (0, n as usize);
        let (mut cols, mut dig1, mut dig2) =
            (vec![false; n], vec![false; 2 * n], vec![false; 2 * n]);
        fn backtrace(
            n: usize,
            x: usize,
            res: &mut i32,
            cols: &mut Vec<bool>,
            dig1: &mut Vec<bool>,
            dig2: &mut Vec<bool>,
        ) {
            if x == n {
                *res += 1;
                return;
            }

            for j in 0..n {
                if cols[j] || dig1[x + j] || dig2[n + x - j] {
                    continue;
                }

                (cols[j], dig1[x + j], dig2[n + x - j]) = (true, true, true);
                backtrace(n, x + 1, res, cols, dig1, dig2);
                (cols[j], dig1[x + j], dig2[n + x - j]) = (false, false, false);
            }
        }

        backtrace(n, 0, &mut res, &mut cols, &mut dig1, &mut dig2);
        res
    }
}

#[test]
fn test() {
    struct Test {
        n: i32,
        expected: i32,
    }

    let tests = vec![
        Test { n: 1, expected: 1 },
        Test { n: 2, expected: 0 },
        Test { n: 3, expected: 0 },
        Test { n: 4, expected: 2 },
        Test { n: 5, expected: 10 },
        Test { n: 6, expected: 4 },
        Test { n: 7, expected: 40 },
        Test { n: 8, expected: 92 },
        Test {
            n: 9,
            expected: 352,
        },
    ];

    for t in tests {
        assert_eq!(Solution::total_n_queens(t.n), t.expected);
    }
}
