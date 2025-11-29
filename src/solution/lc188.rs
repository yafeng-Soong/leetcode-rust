#![allow(dead_code)]
struct Solution;

impl Solution {
    // time complexity is O(n*k), more complicated.
    // buy[i][j] means the maximum profit after j transactions and at the end of day i, if we have bought the stock.
    // sell[i][j] means the maximum profit after j transactions and at the end of day i, if we have sold the stock.
    // buy[i][j] = max(buy[i][j], sell[i-1][j-1] - prices[i])
    // sell[i][j] = max(sell[i-1][j], buy[i-1][j] + prices[i])
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let (k, n) = (k as usize, prices.len());
        let (mut sell, mut buy) = (vec![0; k + 1], vec![0; k + 1]);
        (0..k).for_each(|j| buy[j + 1] = -prices[0]);
        for i in 1..=n {
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j - 1] - prices[i - 1]);
                sell[j] = sell[j].max(buy[j] + prices[i - 1]);
            }
        }

        sell[k]
    }

    // time complexity is O(k*n^2), but more understandable.
    //
    // pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    //     let n = prices.len();
    //     let (mut dp, mut old) = (vec![0; n], vec![0; n]);
    //     let mut min_price = prices[0];
    //     (1..n).for_each(|i| {
    //         min_price = min_price.min(prices[i]);
    //         old[i] = old[i - 1].max(prices[i] - min_price);
    //     });

    //     for _ in 1..k {
    //         for i in 1..n {
    //             dp[i] = dp[i - 1];
    //             for j in (0..=i - 1).rev() {
    //                 dp[i] = dp[i].max(old[j] + prices[i] - prices[j]);
    //             }
    //         }
    //         (old, dp) = (dp, old);
    //     }

    //     old[n - 1]
    // }
}

#[test]
fn test() {
    struct Test {
        k: i32,
        prices: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            k: 2,
            prices: vec![2, 4, 1],
            expected: 2,
        },
        Test {
            k: 2,
            prices: vec![3, 2, 6, 5, 0, 3],
            expected: 7,
        },
        Test {
            k: 1,
            prices: vec![1, 2],
            expected: 1,
        },
    ];

    for t in tests {
        // Solution::max_profit(t.k, t.prices);
        assert_eq!(Solution::max_profit(t.k, t.prices), t.expected);
    }
}
