use crate::solution::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let n = s.len();
        let mut nums = vec![0; n];
        s.into_bytes().iter().enumerate().for_each(|(i, x)| {
            nums[i] = match x {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            }
        });

        for i in 0..n - 1 {
            if nums[i] < nums[i + 1] {
                nums[i] = -nums[i];
            }
        }

        nums.iter().sum()
    }
}

#[test]
fn test() {
    struct Test {
        input: String,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: "III".to_string(),
            expected: 3,
        },
        Test {
            input: "IV".to_string(),
            expected: 4,
        },
        Test {
            input: "IX".to_string(),
            expected: 9,
        },
        Test {
            input: "LVIII".to_string(),
            expected: 58,
        },
        Test {
            input: "MCMXCIV".to_string(),
            expected: 1994,
        },
        Test {
            input: "MMDXLI".to_string(),
            expected: 2541,
        },
    ];

    for t in tests {
        assert_eq!(Solution::roman_to_int(t.input), t.expected);
    }
}
