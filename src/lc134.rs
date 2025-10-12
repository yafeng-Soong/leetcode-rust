use crate::solution::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let (mut sum, mut res, mut cur_sum) = (0, 0, 0);
        for i in 0..n {
            sum += gas[i] - cost[i];
            cur_sum += gas[i] - cost[i];
            if cur_sum < 0 {
                cur_sum = 0;
                res = i as i32 + 1;
            }
        }

        if sum < 0 { -1 } else { res }
    }
}

#[test]
fn test() {
    struct Test {
        gas: Vec<i32>,
        cost: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            gas: vec![1, 2, 3, 4, 5],
            cost: vec![3, 4, 5, 1, 2],
            expected: 3,
        },
        Test {
            gas: vec![2, 3, 4],
            cost: vec![3, 4, 3],
            expected: -1,
        },
    ];

    for t in tests {
        assert_eq!(Solution::can_complete_circuit(t.gas, t.cost), t.expected);
    }
}
