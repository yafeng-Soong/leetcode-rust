use crate::solution::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut res = 0;
        let (mut left, mut right) = (0, x as i64);
        while left <= right {
            let mid = (left + right) / 2;
            if mid * mid > x as i64 {
                right = mid - 1;
            } else {
                res = mid;
                left = mid + 1;
            }
        }

        res as i32
    }
}

#[test]
fn test() {
    struct Test {
        input: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: 4,
            expected: 2,
        },
        Test {
            input: 8,
            expected: 2,
        },
        Test {
            input: 16,
            expected: 4,
        },
        Test {
            input: 26,
            expected: 5,
        },
        Test {
            input: 3785667,
            expected: 1945,
        },
    ];

    for test in tests {
        assert_eq!(Solution::my_sqrt(test.input), test.expected);
    }
}
