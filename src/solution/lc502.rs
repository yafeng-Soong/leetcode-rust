use std::collections::BinaryHeap;

use crate::solution::Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut res = w;
        let n = profits.len();
        let mut pairs = vec![vec![0, 0]; n];
        for i in 0..n {
            pairs[i][0] = profits[i];
            pairs[i][1] = capital[i];
        }

        pairs.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut idx = 0;
        let mut heap = BinaryHeap::new();
        for _ in 0..k {
            while idx < n && res >= pairs[idx][1] {
                heap.push(pairs[idx][0]);
                idx += 1;
            }

            if heap.is_empty() {
                break;
            }

            res += heap.pop().unwrap();
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        k: i32,
        w: i32,
        profits: Vec<i32>,
        capital: Vec<i32>,
    }

    let test_cases = vec![
        Test {
            k: 2,
            w: 0,
            profits: vec![1, 2, 3],
            capital: vec![0, 1, 1],
        },
        Test {
            k: 3,
            w: 0,
            profits: vec![1, 2, 3],
            capital: vec![0, 1, 2],
        },
    ];

    for test in test_cases {
        Solution::find_maximized_capital(test.k, test.w, test.profits, test.capital);
    }
}
