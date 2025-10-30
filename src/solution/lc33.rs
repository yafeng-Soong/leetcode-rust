use crate::solution::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let min_idx = Self::find_min_idx(&nums, 0, n);
        match Self::binary_search(&nums, min_idx, n, target) {
            -1 => Self::binary_search(&nums, 0, min_idx, target),
            res => res,
        }
    }

    fn find_min_idx(nums: &[i32], left: usize, right: usize) -> usize {
        if left >= right || nums[left] < nums[right - 1] {
            return left;
        }

        let mid = (left + right) / 2;
        if nums[mid] > nums[right - 1] {
            return Self::find_min_idx(nums, mid + 1, right);
        }

        let left_min = Self::find_min_idx(nums, left, mid);
        match nums[left_min] < nums[mid] {
            true => left_min,
            false => mid,
        }
    }

    fn binary_search(nums: &[i32], left: usize, right: usize, target: i32) -> i32 {
        if left >= right {
            return -1;
        }

        let mid = (left + right) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => mid as i32,
            Ordering::Less => Self::binary_search(nums, mid + 1, right, target),
            Ordering::Greater => Self::binary_search(nums, left, mid, target),
        }
    }

    // solute with loop.
    pub fn search_loop(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = -1;
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => {
                    res = mid as i32;
                    break;
                }
                Ordering::Greater => {
                    if target < nums[0] && nums[0] <= nums[mid] {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                Ordering::Less => {
                    if nums[mid] <= nums[right - 1] && nums[right - 1] < target {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        nums: Vec<i32>,
        target: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            nums: vec![4, 5, 6, 7, 0, 1, 2],
            target: 0,
            expected: 4,
        },
        Test {
            nums: vec![4, 5, 6, 7, 0, 1, 2],
            target: 3,
            expected: -1,
        },
        Test {
            nums: vec![1],
            target: 0,
            expected: -1,
        },
        Test {
            nums: vec![3, 1],
            target: 3,
            expected: 0,
        },
    ];

    for test in tests {
        assert_eq!(
            Solution::search(test.nums.clone(), test.target),
            test.expected
        );
        assert_eq!(Solution::search_loop(test.nums, test.target), test.expected);
    }
}
