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
