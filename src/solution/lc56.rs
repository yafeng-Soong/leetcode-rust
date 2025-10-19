use crate::solution::Solution;
use std::cmp::max;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        intervals.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut left = 0;
        let mut right = 0;
        let n = intervals.len();
        while left < n {
            let mut upper = intervals[left][1];
            while right < n && intervals[right][0] <= upper {
                upper = max(upper, intervals[right][1]);
                right += 1;
            }

            res.push(vec![intervals[left][0], upper]);
            left = right;
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        intervals: Vec<Vec<i32>>,
        expect: Vec<Vec<i32>>,
    }
    let tests = vec![
        Test {
            intervals: vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            expect: vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        },
        Test {
            intervals: vec![vec![1, 4], vec![4, 5]],
            expect: vec![vec![1, 5]],
        },
        Test {
            intervals: vec![vec![1, 4], vec![0, 4]],
            expect: vec![vec![0, 4]],
        },
        Test {
            intervals: vec![
                vec![5, 7],
                vec![0, 0],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![3, 5],
            ],
            expect: vec![vec![0, 0], vec![2, 2], vec![3, 7]],
        },
        Test {
            intervals: vec![vec![1, 4], vec![0, 2], vec![3, 5]],
            expect: vec![vec![0, 5]],
        },
    ];

    for test in tests {
        assert_eq!(Solution::merge(test.intervals), test.expect);
    }
}
