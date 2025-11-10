use crate::solution::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut romans = Vec::new();
        let mut num = num;
        let mut i: usize = 1;
        while num > 0 {
            let x = (num % 10) as usize;
            romans.insert(
                0,
                match (x, i) {
                    (y, 1) if y <= 3 => "I".repeat(y),
                    (4, 1) => "IV".to_string(),
                    (y, 1) if y > 4 && y < 9 => String::from("V") + &"I".repeat(y - 5),
                    (9, 1) => "IX".to_string(),
                    (y, 10) if y <= 3 => "X".repeat(y),
                    (4, 10) => "XL".to_string(),
                    (y, 10) if y > 4 && y < 9 => String::from("L") + &"X".repeat(y - 5),
                    (9, 10) => "XC".to_string(),
                    (y, 100) if y <= 3 => "C".repeat(y),
                    (4, 100) => "CD".to_string(),
                    (y, 100) if y > 4 && y < 9 => String::from("D") + &"C".repeat(y - 5),
                    (9, 100) => "CM".to_string(),
                    (y, 1000) if y <= 3 => "M".repeat(y),
                    (_, _) => String::new(),
                },
            );

            (i, num) = (i * 10, num / 10);
        }

        romans.join("")
    }
}
