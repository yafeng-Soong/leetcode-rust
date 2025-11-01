use crate::solution::Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            _ => (n & 1) + Solution::hamming_weight(n >> 1),
        }
    }
}

#[test]
fn test() {
    struct Test {
        n: i32,
        expected: i32,
    }

    let tests = vec![
        Test { n: 11, expected: 3 },
        Test {
            n: 128,
            expected: 1,
        },
        Test {
            n: 2147483645,
            expected: 30,
        },
    ];

    for t in tests {
        assert_eq!(Solution::hamming_weight(t.n), t.expected);
    }
}
