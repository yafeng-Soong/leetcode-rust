use crate::solution::Solution;

impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let (mut res, mut left, mut right) = (0, 0, nums.len() - 1);
        nums.sort();
        while left < right {
            match (nums[left] + nums[right]).cmp(&k) {
                std::cmp::Ordering::Equal => {
                    res += 1;
                    left += 1;
                    right -= 1;
                }
                std::cmp::Ordering::Less => {
                    left += 1;
                }
                std::cmp::Ordering::Greater => {
                    right -= 1;
                }
            }
        }

        res
    }
}

#[test]
fn test() {
    struct TestCase {
        nums: Vec<i32>,
        k: i32,
        expected: i32,
    }

    let test_cases = [
        TestCase {
            nums: vec![1, 2, 3, 4],
            k: 5,
            expected: 2,
        },
        TestCase {
            nums: vec![3, 1, 3, 4, 3],
            k: 6,
            expected: 1,
        },
    ];

    for tc in test_cases {
        assert_eq!(Solution::max_operations(tc.nums, tc.k), tc.expected);
    }
}
