#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let (mut dp, mut old) = (vec![0; n], vec![0; n]);
        let mut min_price = prices[0];
        (1..n).for_each(|i| {
            min_price = min_price.min(prices[i]);
            old[i] = old[i - 1].max(prices[i] - min_price);
        });

        for _ in 1..k {
            for i in 1..n {
                dp[i] = dp[i - 1];
                for j in (0..=i - 1).rev() {
                    dp[i] = dp[i].max(old[j] + prices[i] - prices[j]);
                }
            }
            (old, dp) = (dp, old);
        }

        // max_profit.iter().for_each(|x| println!("{:?}", x));
        old[n - 1]
    }
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
