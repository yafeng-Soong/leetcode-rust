use crate::solution::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut romans = Vec::new();
        let mut num = num;
        let base_char = &[
            &["I", "V", "X"],
            &["X", "L", "C"],
            &["C", "D", "M"],
            &["M", "M", "M"],
        ];
        let mut iter = base_char.iter();
        while num > 0 {
            let x = (num % 10) as usize;
            let base = iter.next().unwrap();
            romans.insert(
                0,
                match x {
                    y if y <= 3 => base[0].repeat(y),
                    4 => String::from(base[0]) + base[1],
                    y if y > 4 && y < 9 => String::from(base[1]) + &base[0].repeat(y - 5),
                    9 => String::from(base[0]) + base[2],
                    _ => String::new(),
                },
            );

            num /= 10;
        }

        romans.join("")
    }
}

#[test]
fn test() {
    struct Test {
        num: i32,
        expected: String,
    }

    let tests = vec![
        Test {
            num: 3,
            expected: "III".to_string(),
        },
        Test {
            num: 4,
            expected: "IV".to_string(),
        },
        Test {
            num: 9,
            expected: "IX".to_string(),
        },
        Test {
            num: 58,
            expected: "LVIII".to_string(),
        },
        Test {
            num: 1994,
            expected: "MCMXCIV".to_string(),
        },
        Test {
            num: 3020,
            expected: "MMMXX".to_string(),
        },
    ];

    for t in tests {
        assert_eq!(Solution::int_to_roman(t.num), t.expected);
    }
}
