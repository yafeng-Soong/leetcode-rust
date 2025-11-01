use crate::solution::Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let x = 1 << (31 - i);
            match (left & x, right & x) {
                (0, 0) => (),
                (a, b) if a == b => res |= x,
                (_, _) => break,
            }
        }

        res
    }
}
