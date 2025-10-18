use crate::solution::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for row in matrix {
            for num in row {
                if heap.len() == k as usize && num >= *heap.peek().unwrap() {
                    break;
                }

                heap.push(num);
                if heap.len() > k as usize {
                    heap.pop();
                }
            }
        }

        heap.pop().unwrap()
    }
}

#[test]
fn test_kth_smallest() {
    struct Test {
        matrix: Vec<Vec<i32>>,
        k: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            matrix: vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]],
            k: 8,
            expected: 13,
        },
        Test {
            matrix: vec![vec![-5]],
            k: 1,
            expected: -5,
        },
    ];

    for test in tests {
        assert_eq!(Solution::kth_smallest(test.matrix, test.k), test.expected);
    }
}
