use crate::solution::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![0; n];
        for (i, x) in dp.iter_mut().enumerate() {
            if obstacle_grid[0][i] == 1 {
                break;
            }

            *x = 1;
        }

        (1..m).for_each(|i| {
            dp[0] = if obstacle_grid[i][0] == 1 { 0 } else { dp[0] };
            (1..n).for_each(|j| {
                dp[j] = if obstacle_grid[i][j] == 1 {
                    0
                } else {
                    dp[j - 1] + dp[j]
                };
            });
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
            input: vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            expected: 2,
        },
        Test {
            input: vec![vec![0, 1], vec![0, 0]],
            expected: 1,
        },
    ];

    for t in tests {
        assert_eq!(Solution::unique_paths_with_obstacles(t.input), t.expected);
    }
}
