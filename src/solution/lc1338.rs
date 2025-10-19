use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        for &val in &arr {
            *count_map.entry(val).or_insert(0) += 1;
        }

        let mut nums: Vec<i32> = count_map.values().cloned().collect();
        nums.sort_by(|a, b| b.cmp(a));
        let half = (arr.len() / 2) as i32;
        let mut res = 0;
        let mut sum = 0;
        for val in nums {
            res += 1;
            sum += val;
            if sum >= half {
                break;
            }
        }

        res
    }
}

#[test]
fn test_reduce() {
    struct Test {
        arr: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            arr: vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7],
            expected: 2,
        },
        Test {
            arr: vec![7, 7, 7, 7, 7],
            expected: 1,
        },
    ];

    for t in tests {
        let res = Solution::min_set_size(t.arr);
        assert_eq!(t.expected, res);
    }
}
