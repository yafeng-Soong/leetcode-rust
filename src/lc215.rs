use std::collections::BinaryHeap;

use crate::solution::Solution;

const MAX_VALUE: i32 = 10000;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnts = [0; 2 * (MAX_VALUE as usize) + 1];
        for num in nums {
            cnts[(num + MAX_VALUE) as usize] += 1;
        }

        let mut res = 0;
        let mut idx = 2 * MAX_VALUE;
        let mut cnt = k;
        while cnt > 0 {
            res = idx;
            cnt -= cnts[idx as usize];
            idx -= 1;
        }

        res - MAX_VALUE
    }
}
