use crate::solution::Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut n = ratings.iter().len();
        let mut cnts = vec![1; ratings.len()];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                cnts[i] = cnts[i - 1] + 1;
            }
        }

        for i in (1..n).rev() {
            if ratings[i - 1] > ratings[i] {
                cnts[i - 1] = cnts[i - 1].max(cnts[i] + 1);
            }
        }

        cnts.iter().sum()
    }
}

#[test]
fn test() {
    struct Test {
        ratings: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            ratings: vec![1, 0, 2],
            expected: 5,
        },
        Test {
            ratings: vec![1, 2, 2],
            expected: 4,
        },
        Test {
            ratings: vec![1, 3, 4, 5, 2],
            expected: 11,
        },
        Test {
            ratings: vec![1, 3, 2, 2, 1],
            expected: 7,
        },
    ];

    for t in tests {
        assert_eq!(Solution::candy(t.ratings), t.expected);
    }
}
