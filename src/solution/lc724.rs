use crate::solution::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums.iter().sum();
        let mut pre_sum = 0;
        for (i, num) in nums.iter().enumerate() {
            sum -= num;
            if pre_sum == sum {
                return i as i32;
            }

            pre_sum += num;
        }
        -1
    }
}

#[test]
fn test() {
    struct Test {
        input: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: vec![1, 7, 3, 6, 5, 6],
            expected: 3,
        },
        Test {
            input: vec![1, 2, 3],
            expected: -1,
        },
        Test {
            input: vec![2, 1, -1],
            expected: 0,
        },
    ];

    for t in tests {
        assert_eq!(Solution::pivot_index(t.input), t.expected);
    }
}
