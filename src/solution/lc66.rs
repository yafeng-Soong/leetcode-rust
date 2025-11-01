use crate::solution::Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let (mut idx, mut up) = (digits.len(), 1);
        while idx > 0 && up != 0 {
            digits[idx - 1] += up;
            up = digits[idx - 1] / 10;
            digits[idx - 1] %= 10;
            idx -= 1;
        }

        if up != 0 {
            digits.insert(0, 1);
        }

        digits
    }
}

#[test]
fn test() {
    struct Test {
        digits: Vec<i32>,
        expected: Vec<i32>,
    }

    let tests = vec![
        Test {
            digits: vec![1, 2, 3],
            expected: vec![1, 2, 4],
        },
        Test {
            digits: vec![4, 3, 2, 1],
            expected: vec![4, 3, 2, 2],
        },
        Test {
            digits: vec![9],
            expected: vec![1, 0],
        },
    ];

    for t in tests {
        assert_eq!(Solution::plus_one(t.digits), t.expected);
    }
}
