use crate::solution::Solution;
use std::cmp::Ordering;

enum EdgeType {
    Lower,
    Upper,
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![
            Self::edge_binary_search(&nums, target, EdgeType::Lower),
            Self::edge_binary_search(&nums, target, EdgeType::Upper),
        ]
    }

    fn edge_binary_search(nums: &[i32], target: i32, edge_type: EdgeType) -> i32 {
        let mut res = -1;
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2; // incase overflow
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
                Ordering::Equal => {
                    res = mid as i32;
                    match edge_type {
                        EdgeType::Lower => right = mid,
                        EdgeType::Upper => left = mid + 1,
                    }
                }
            }
        }

        res
    }

    // more understanding solution:
    // fn edge_binary_search(nums: &[i32], target: i32, upper: bool) -> i32 {
    //     let mut res = -1;
    //     let (mut left, mut right) = (0, nums.len());
    //     while left < right {
    //         let mid = left + (right - left) / 2;
    //         if nums[mid] == target {
    //             res = mid as i32;
    //         }

    //         if nums[mid] < target || (nums[mid] == target && upper) {
    //             left = mid + 1;
    //             continue;
    //         }

    //         right = mid;
    //     }

    //     res
    // }
}

#[test]
fn test() {
    struct Test {
        nums: Vec<i32>,
        target: i32,
        expected: Vec<i32>,
    }

    let tests = vec![
        Test {
            nums: vec![5, 7, 7, 8, 8, 10],
            target: 8,
            expected: vec![3, 4],
        },
        Test {
            nums: vec![5, 7, 7, 8, 8, 10],
            target: 6,
            expected: vec![-1, -1],
        },
        Test {
            nums: vec![],
            target: 0,
            expected: vec![-1, -1],
        },
    ];

    for t in tests {
        assert_eq!(Solution::search_range(t.nums, t.target), t.expected);
    }
}
