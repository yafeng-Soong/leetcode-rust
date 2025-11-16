use crate::solution::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();
        fn op_num(a: i32, b: i32, op: &str) -> i32 {
            match op {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            }
        }

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop_back().unwrap();
                    let a = stack.pop_back().unwrap();
                    stack.push_back(op_num(a, b, token.as_str()));
                }
                _ => stack.push_back(token.parse().unwrap()),
            }
        }

        stack.pop_back().unwrap()
    }
}

#[test]
fn test() {
    struct Test {
        tokens: Vec<&'static str>,
        expected: i32,
    }

    let tests = vec![
        Test {
            tokens: vec!["2", "1", "+", "3", "*"],
            expected: 9,
        },
        Test {
            tokens: vec!["4", "13", "5", "/", "+"],
            expected: 6,
        },
        Test {
            tokens: vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
            ],
            expected: 22,
        },
        Test {
            tokens: vec!["3", "-4", "+"],
            expected: -1,
        },
    ];

    for t in tests {
        let tokens = t.tokens.iter().map(|&s| s.to_string()).collect();
        assert_eq!(Solution::eval_rpn(tokens), t.expected);
    }
}
