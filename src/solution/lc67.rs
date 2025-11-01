use crate::solution::Solution;
use std::cmp;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a_len, b_len, mut up) = (a.len(), b.len(), 0);
        let n = cmp::max(a_len, b_len);
        let (a_bytes, b_bytes) = (a.into_bytes(), b.into_bytes());
        let mut res_bytes = vec![b'0'; n];
        for i in 1..=n {
            if i <= a_len {
                up += a_bytes[a_len - i] - b'0';
            }

            if i <= b_len {
                up += b_bytes[b_len - i] - b'0';
            }

            res_bytes[n - i] = up % 2 + b'0';
            up /= 2;
        }

        let res = String::from_utf8(res_bytes).unwrap();
        match up {
            0 => res,
            _ => String::from("1") + &res,
        }
    }
}

#[test]
fn test() {
    struct Test {
        a: String,
        b: String,
        expected: String,
    }

    let tests = vec![
        Test {
            a: String::from("11"),
            b: String::from("1"),
            expected: String::from("100"),
        },
        Test {
            a: String::from("1010"),
            b: String::from("1011"),
            expected: String::from("10101"),
        },
    ];

    for test in tests {
        assert_eq!(Solution::add_binary(test.a, test.b), test.expected);
    }
}
