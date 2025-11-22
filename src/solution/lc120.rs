use crate::solution::Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = triangle[0][0];
        (1..n).for_each(|i| {
            for j in (1..=i).rev() {
                dp[j] = dp[j].min(dp[j - 1]) + triangle[i][j];
            }

            dp[0] += triangle[i][0];
        });

        dp.into_iter().min().unwrap()
    }
}

#[test]
fn test() {
    struct Test {
        triangle: Vec<Vec<i32>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            triangle: vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
            expected: 11,
        },
        Test {
            triangle: vec![vec![-10]],
            expected: -10,
        },
    ];

    for t in tests {
        assert_eq!(Solution::minimum_total(t.triangle), t.expected);
    }
}
