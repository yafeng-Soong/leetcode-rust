use crate::solution::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut res, mut idx, n) = (0, 0, prices.len());
        while idx < n && idx + 1 < n {
            if prices[idx] < prices[idx + 1] {
                res += prices[idx + 1] - prices[idx];
            }

            idx += 1;
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
            prices: vec![7, 1, 5, 3, 6, 4],
            expected: 7,
        },
        Test {
            prices: vec![1, 2, 3, 4, 5],
            expected: 4,
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
