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

            if heap.len() == 0 {
                break;
            }

            println!("{:?}", heap);
            res += heap.pop().unwrap();
        }
        res
    }
}

#[test]
fn test() {
    Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]);
}
