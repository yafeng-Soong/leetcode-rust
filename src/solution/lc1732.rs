use crate::solution::Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut res, mut sum) = (0, 0);
        gain.iter().for_each(|x| {
            sum += x;
            res = res.max(sum);
        });

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
            input: vec![-5, 1, 5, 0, -7],
            expected: 1,
        },
        Test {
            input: vec![-4, -3, -2, -1, 4, 3, 2],
            expected: 0,
        },
    ];

    for t in tests {
        assert_eq!(Solution::largest_altitude(t.input), t.expected);
    }
}
