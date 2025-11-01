use crate::solution::Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            if n & (1 << i) != 0 {
                res |= 1 << (31 - i);
            }
        }

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
        Test {
            n: 43261596,
            expected: 964176192,
        },
        Test {
            n: 2147483644,
            expected: 1073741822,
        },
    ];

    for t in tests {
        assert_eq!(Solution::reverse_bits(t.n), t.expected);
    }
}
