#![allow(clippy::ptr_arg)]
use crate::solution::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (n, mut end) = (nums.len(), 0);
        for p in 0..n {
            if nums[p] == 0 {
                continue;
            }

            nums.swap(end, p);
            end += 1;
        }
    }
}

#[test]
fn test() {
    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<i32>,
    }

    let test_cases = [
        TestCase {
            nums: vec![0, 1, 0, 3, 12],
            expected: vec![1, 3, 12, 0, 0],
        },
        TestCase {
            nums: vec![0],
            expected: vec![0],
        },
    ];

    for tc in test_cases {
        let mut nums = tc.nums.clone();
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, tc.expected);
    }
}
