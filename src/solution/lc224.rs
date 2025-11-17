use crate::solution::Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.replace(" ", "");
        let tokens = s.as_bytes();

        Self::calculate_tokens(tokens)
    }

    fn calculate_tokens(tokens: &[u8]) -> i32 {
        let mut nums = Vec::new();
        let (mut idx, mut negtive, n) = (0, false, tokens.len());
        while idx < n {
            match tokens[idx] {
                b'-' => {
                    negtive = true;
                    idx += 1;
                }
                b'+' => {
                    negtive = false;
                    idx += 1;
                }
                b'(' => {
                    let mut sum = 1;
                    let mut offset = 0;
                    while sum != 0 {
                        offset += 1;
                        if tokens[idx + offset] == b'(' {
                            sum += 1;
                        } else if tokens[idx + offset] == b')' {
                            sum -= 1;
                        }
                    }

                    let num = Self::calculate_tokens(&tokens[idx + 1..idx + offset]);
                    idx += offset + 1;

                    if negtive {
                        nums.push(-num);
                    } else {
                        nums.push(num);
                    }
                }
                _ => {
                    let mut num = 0;
                    while idx < n && tokens[idx].is_ascii_digit() {
                        num = num * 10 + (tokens[idx] - b'0') as i32;
                        idx += 1;
                    }

                    if negtive {
                        nums.push(-num);
                    } else {
                        nums.push(num);
                    }
                }
            }
        }

        nums.iter().sum()
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
            input: "1 + 1".to_string(),
            expected: 2,
        },
        Test {
            input: " 2-1 + 2 ".to_string(),
            expected: 3,
        },
        Test {
            input: "(1+(4+5+2)-3)+(6+8)".to_string(),
            expected: 23,
        },
        Test {
            input: "1-( -2)".to_string(),
            expected: 3,
        },
        Test {
            input: "2147483647".to_string(),
            expected: 2147483647,
        },
    ];

    for t in tests {
        assert_eq!(Solution::calculate(t.input), t.expected);
    }
}
