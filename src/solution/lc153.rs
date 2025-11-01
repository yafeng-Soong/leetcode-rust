use crate::solution::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right && nums[left] > nums[right - 1] {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&nums[right - 1]) {
                std::cmp::Ordering::Less => right = mid + 1,
                std::cmp::Ordering::Greater => left = mid + 1,
                std::cmp::Ordering::Equal => left = mid,
            }
        }

        nums[left]
    }
}

#[test]
fn test() {
    struct Test {
        nums: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            nums: vec![3, 4, 5, 1, 2],
            expected: 1,
        },
        Test {
            nums: vec![4, 5, 6, 7, 0, 1, 2],
            expected: 0,
        },
        Test {
            nums: vec![11, 13, 15, 17],
            expected: 11,
        },
        Test {
            nums: vec![3, 1],
            expected: 1,
        },
    ];

    for t in tests {
        assert_eq!(Solution::find_min(t.nums), t.expected);
    }
}
