use crate::solution::Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let x = 1 << (31 - i);
            match (left & x, right & x) {
                (0, 0) => (),
                (a, b) if a == b => res |= x,
                (_, _) => break,
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        left: i32,
        right: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            left: 5,
            right: 7,
            expected: 4,
        },
        Test {
            left: 0,
            right: 0,
            expected: 0,
        },
        Test {
            left: 1,
            right: 2,
            expected: 0,
        },
    ];

    for t in tests {
        assert_eq!(Solution::range_bitwise_and(t.left, t.right), t.expected);
    }
}
