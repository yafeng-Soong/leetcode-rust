use crate::solution::Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let x = coordinates.chars().nth(0).unwrap() as i32;
        let y = coordinates.chars().nth(1).unwrap() as i32;

        x % 2 != y % 2
    }
}

#[test]
fn test_lc1812() {
    struct Test {
        input: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: String::from("a1"),
            expected: false,
        },
        Test {
            input: String::from("h3"),
            expected: true,
        },
    ];

    for test in tests {
        let mut res = Solution::square_is_white(test.input);
        assert_eq!(test.expected, res);
    }
}
