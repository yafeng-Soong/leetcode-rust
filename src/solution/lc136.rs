#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.iter().for_each(|x| res ^= x);
        res
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
            input: vec![2, 2, 1],
            expected: 1,
        },
        Test {
            input: vec![4, 1, 2, 1, 2],
            expected: 4,
        },
        Test {
            input: vec![1],
            expected: 1,
        },
    ];

    for t in tests {
        assert_eq!(Solution::single_number(t.input), t.expected);
    }
}
