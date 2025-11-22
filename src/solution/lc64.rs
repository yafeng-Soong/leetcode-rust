use crate::solution::Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![0; n];
        dp[0] = grid[0][0];
        (1..n).for_each(|i| dp[i] = dp[i - 1] + grid[0][i]);
        (1..m).for_each(|i| {
            dp[0] += grid[i][0];
            (1..n).for_each(|j| dp[j] = dp[j].min(dp[j - 1]) + grid[i][j]);
        });

        dp[n - 1]
    }
}

#[test]
fn test() {
    struct Test {
        input: Vec<Vec<i32>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]],
            expected: 7,
        },
        Test {
            input: vec![vec![1, 2, 3], vec![4, 5, 6]],
            expected: 12,
        },
    ];

    for t in tests {
        assert_eq!(Solution::min_path_sum(t.input), t.expected);
    }
}
