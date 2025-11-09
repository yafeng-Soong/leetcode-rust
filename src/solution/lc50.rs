use crate::solution::Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn pow(x: f64, n: i64) -> f64 {
            if n == 0 {
                return 1.0;
            }

            let y = pow(x, n / 2);
            if n % 2 == 0 {
                return y * y;
            }

            x * y * y
        }

        let n = n as i64;
        if n < 0 {
            return 1.0 / pow(x, n);
        }

        pow(x, n)
    }
}

#[test]
fn test() {
    struct Test {
        x: f64,
        n: i32,
        expected: f64,
    }

    let tests = vec![
        Test {
            x: 2.0,
            n: 10,
            expected: 1024.0,
        },
        Test {
            x: 2.1,
            n: 3,
            expected: 9.26100,
        },
        Test {
            x: 2.0,
            n: -2,
            expected: 0.25,
        },
        Test {
            x: -1.0,
            n: -2147483648,
            expected: 1.0,
        },
        Test {
            x: 2.0,
            n: -2147483648,
            expected: 0.00000,
        },
        Test {
            x: 1.0,
            n: 2147483647,
            expected: 1.00000,
        },
    ];

    for t in tests {
        assert_eq!(
            format!("{:.5}", Solution::my_pow(t.x, t.n)),
            format!("{:.5}", t.expected)
        );
    }
}
