#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut res, n) = (0, prices.len());
        let mut min_price = prices[0];
        (1..n).for_each(|i| {
            min_price = min_price.min(prices[i]);
            res = res.max(prices[i] - min_price);
        });

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
            prices: vec![7, 1, 5, 3, 6, 4],
            expected: 5,
        },
        Test {
            prices: vec![7, 6, 4, 3, 1],
            expected: 0,
        },
    ];

    for t in tests {
        assert_eq!(Solution::max_profit(t.prices), t.expected);
    }
}
