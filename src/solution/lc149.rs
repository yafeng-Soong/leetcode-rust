use crate::solution::Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n < 3 {
            return n as i32;
        }

        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                let mut cnt = 2;
                for k in j + 1..n {
                    let (x1, x2, y1, y2) = (points[i][0], points[j][0], points[i][1], points[j][1]);
                    let (x, y) = (points[k][0], points[k][1]);
                    if (y2 - y1) * (x - x1) == (y - y1) * (x2 - x1) {
                        cnt += 1;
                    }
                }
                res = res.max(cnt);
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        points: Vec<Vec<i32>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            points: vec![vec![1, 1], vec![2, 2], vec![3, 3]],
            expected: 3,
        },
        Test {
            points: vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4],
            ],
            expected: 4,
        },
        Test {
            points: vec![vec![4, 5], vec![4, -1], vec![4, 0]],
            expected: 3,
        },
        Test {
            points: vec![vec![0, 0]],
            expected: 1,
        },
    ];

    for t in tests {
        assert_eq!(Solution::max_points(t.points), t.expected);
    }
}
