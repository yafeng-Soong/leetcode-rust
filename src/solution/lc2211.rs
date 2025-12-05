use crate::solution::Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut res = 0;
        let mut left = -1;
        for c in directions.chars() {
            match c {
                'R' => {
                    if left == -1 {
                        left += 1;
                    }

                    left += 1;
                }
                'L' => {
                    if left != -1 {
                        res += left + 1;
                        left = 0;
                    }
                }
                'S' => {
                    if left != -1 {
                        res += left;
                    }

                    left = 0;
                }
                _ => unreachable!(),
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        input: String,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: "RLRSLL".to_string(),
            expected: 5,
        },
        Test {
            input: "LLRR".to_string(),
            expected: 0,
        },
        Test {
            input: "RRLL".to_string(),
            expected: 4,
        },
        Test {
            input: "LLLRSSRSRRRLRRLLSSLSLLRLLSRRRRRRSLLRRLLLSRRSSSS".to_string(),
            expected: 31,
        },
    ];

    for t in tests {
        assert_eq!(Solution::count_collisions(t.input), t.expected);
    }
}
