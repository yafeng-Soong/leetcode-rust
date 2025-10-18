use std::{cmp::min, collections::VecDeque};

use crate::solution::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = height.len();
        let mut right_idx = vec![0; n];
        let mut stack = VecDeque::new();
        for i in 0..n {
            while let Some(&top) = stack.back() {
                if height[top] > height[i] {
                    break;
                }
                right_idx[top] = i;
                stack.pop_back();
            }

            stack.push_back(i);
        }

        while stack.len() > 1 {
            let top = stack.pop_back().unwrap();
            let &left = stack.back().unwrap();
            right_idx[left] = top;
        }

        let (mut idx, mut next) = (0, 0);
        while idx < n - 1 {
            next = right_idx[idx];
            let h = min(height[idx], height[next]);
            for i in idx + 1..next {
                res += h - height[i];
            }
            idx = next;
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        height: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            height: vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            expected: 6,
        },
        Test {
            height: vec![4, 2, 0, 3, 2, 5],
            expected: 9,
        },
    ];

    for t in tests {
        assert_eq!(Solution::trap(t.height), t.expected);
    }
}
