use crate::solution::Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut bits = &mut [0; 32];
        for mut num in nums {
            bits.iter_mut().for_each(|x| {
                *x += num & 1;
                num >>= 1;
            });
        }

        for (i, bit) in bits.iter().enumerate() {
            if bit % 3 != 0 {
                res |= 1 << i;
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        nums: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            nums: vec![2, 2, 3, 2],
            expected: 3,
        },
        Test {
            nums: vec![0, 1, 0, 1, 0, 1, 99],
            expected: 99,
        },
    ];

    for t in tests {
        assert_eq!(Solution::single_number(t.nums), t.expected);
    }
}
