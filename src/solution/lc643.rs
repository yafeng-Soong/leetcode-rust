use crate::solution::Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let (mut sum, mut res) = (0, 0);
        let k = k as usize;
        let n = nums.len();

        sum = nums.iter().take(k).sum();
        res = sum;
        for i in k..n {
            sum += nums[i];
            sum -= nums[i - k];
            res = res.max(sum);
        }

        (res as f64) / (k as f64)
    }
}

#[test]
fn test() {
    struct TestCase {
        nums: Vec<i32>,
        k: i32,
        expected: f64,
    }

    let test_cases = [
        TestCase {
            nums: vec![1, 12, -5, -6, 50, 3],
            k: 4,
            expected: 12.75,
        },
        TestCase {
            nums: vec![5],
            k: 1,
            expected: 5.0,
        },
    ];

    for tc in test_cases {
        assert_eq!(Solution::find_max_average(tc.nums, tc.k), tc.expected);
    }
}
