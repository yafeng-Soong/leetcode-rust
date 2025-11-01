use crate::solution::Solution;

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n / 5;
            n /= 5;
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
        Test { n: 3, expected: 0 },
        Test { n: 5, expected: 1 },
        Test { n: 10, expected: 2 },
        Test { n: 25, expected: 6 },
        Test {
            n: 125,
            expected: 31,
        },
        Test {
            n: 9875,
            expected: 2467,
        },
        Test {
            n: 10000,
            expected: 2499,
        },
    ];

    for t in tests {
        assert_eq!(Solution::trailing_zeroes(t.n), t.expected);
    }
}
