use crate::solution::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut res, mut left, mut right) = (0, 0, height.len() - 1);
        while left < right {
            res = res.max((right - left) as i32 * height[left].min(height[right]));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        res
    }
}

#[test]
fn test_max_area() {
    struct TestCase {
        height: Vec<i32>,
        expected: i32,
    }

    let test_cases = [
        TestCase {
            height: vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
            expected: 49,
        },
        TestCase {
            height: vec![1, 1],
            expected: 1,
        },
    ];

    for tc in test_cases {
        assert_eq!(Solution::max_area(tc.height), tc.expected);
    }
}
