#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (n, mut res) = (prices.len(), 0);
        let mut pre_profit = vec![0; n + 1];
        let (mut prev, mut post, mut post_max_profit) = (1, 1, 0);
        while post <= n {
            let delta = prices[post - 1] - prices[prev - 1];
            pre_profit[post] = pre_profit[post - 1].max(delta);
            if delta < 0 {
                prev = post;
            }

            post += 1;
        }

        (prev, post) = (n, n);
        while prev > 0 {
            let delta = prices[post - 1] - prices[prev - 1];
            post_max_profit = post_max_profit.max(delta);
            res = res.max(post_max_profit + pre_profit[prev]);
            if delta < 0 {
                post = prev;
            }

            prev -= 1;
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        prices: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            prices: vec![3, 3, 5, 0, 0, 3, 1, 4],
            expected: 6,
        },
        Test {
            prices: vec![1, 2, 3, 4, 5],
            expected: 4,
        },
        Test {
            prices: vec![7, 6, 4, 3, 1],
            expected: 0,
        },
        Test {
            prices: vec![1],
            expected: 0,
        },
    ];

    for t in tests {
        assert_eq!(Solution::max_profit(t.prices), t.expected);
    }
}
