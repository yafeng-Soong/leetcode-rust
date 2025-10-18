use crate::solution::Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let n = intervals.len();
        let (mut left, mut right) = (-1, n);
        for i in 0..n {
            if intervals[i][1] < new_interval[0] {
                left = i as i32;
                continue;
            }

            if intervals[i][0] > new_interval[1] {
                right = i;
                break;
            }
        }

        match ((left + 1) as usize, right) {
            // insert into left side.
            (_, 0) => {
                res.push(new_interval);
                res.extend(intervals);
            }
            // insert into right side.
            (val, _) if val == n => {
                res.extend(intervals);
                res.push(new_interval);
            }
            // merge and insert to middle.
            (left, right) => {
                res.extend(intervals[0..left].iter().cloned());
                let lower = intervals[left][0].min(new_interval[0]);
                let upper = intervals[right - 1][1].max(new_interval[1]);
                res.push(vec![lower, upper]);
                res.extend(intervals[right..].iter().cloned());
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        intervals: Vec<Vec<i32>>,
        new_interval: Vec<i32>,
        expected: Vec<Vec<i32>>,
    }

    let tests = vec![
        Test {
            intervals: vec![vec![1, 3], vec![6, 9]],
            new_interval: vec![2, 5],
            expected: vec![vec![1, 5], vec![6, 9]],
        },
        Test {
            intervals: vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            new_interval: vec![4, 8],
            expected: vec![vec![1, 2], vec![3, 10], vec![12, 16]],
        },
        Test {
            intervals: vec![vec![1, 5]],
            new_interval: vec![6, 8],
            expected: vec![vec![1, 5], vec![6, 8]],
        },
    ];

    for t in tests {
        assert_eq!(Solution::insert(t.intervals, t.new_interval), t.expected);
    }
}
