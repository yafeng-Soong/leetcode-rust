use crate::solution::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let (mut reverse, mut num) = (0, x);
        while num > 0 {
            reverse = reverse * 10 + num % 10;
            num /= 10;
        }

        reverse == x
    }
}

#[test]
fn test() {
    struct Test {
        x: i32,
        expected: bool,
    }

    let tests = vec![
        Test {
            x: 121,
            expected: true,
        },
        Test {
            x: -121,
            expected: false,
        },
        Test {
            x: 10,
            expected: false,
        },
    ];

    for t in tests {
        assert_eq!(Solution::is_palindrome(t.x), t.expected);
    }
}
