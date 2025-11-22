use crate::solution::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];
        for _ in 1..m {
            (1..n).for_each(|j| dp[j] += dp[j - 1]);
        }

        dp[n - 1]
    }
}

#[test]
fn test() {
    struct Test {
        m: i32,
        n: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            m: 3,
            n: 2,
            expected: 3,
        },
        Test {
            m: 7,
            n: 3,
            expected: 28,
        },
    ];

    for t in tests {
        assert_eq!(Solution::unique_paths(t.m, t.n), t.expected);
    }
}
