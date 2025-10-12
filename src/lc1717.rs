use std::collections::VecDeque;

use crate::solution::Solution;
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (count_x, count_y): (i32, i32);
        let mut chars = s.as_bytes().to_vec();
        if x >= y {
            (chars, count_x) = Self::erease(chars, 'a' as u8, 'b' as u8);
            (_, count_y) = Self::erease(chars, 'b' as u8, 'a' as u8);
        } else {
            (chars, count_y) = Self::erease(chars, 'b' as u8, 'a' as u8);
            (_, count_x) = Self::erease(chars, 'a' as u8, 'b' as u8);
        }

        x * count_x + y * count_y
    }

    fn erease(chars: Vec<u8>, lc: u8, rc: u8) -> (Vec<u8>, i32) {
        let mut count = 0;
        let mut stack = VecDeque::new();
        for c in chars {
            if stack.len() > 0 && stack.back() == Some(&lc) && c == rc {
                stack.pop_back();
                count += 1;
                continue;
            }

            stack.push_back(c);
        }

        let chars = stack.iter().map(|x| *x).collect::<Vec<_>>();
        (chars, count)
    }
}

#[test]
fn test_maximum_gain() {
    struct Test {
        s: String,
        x: i32,
        y: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            s: "cdbcbbaaabab".to_string(),
            x: 4,
            y: 5,
            expected: 19,
        },
        Test {
            s: "aabbaaxybbaabb".to_string(),
            x: 5,
            y: 4,
            expected: 20,
        },
    ];

    for t in tests {
        assert_eq!(Solution::maximum_gain(t.s, t.x, t.y), t.expected);
    }
}
