use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut cnts = HashMap::new();
        for i in 0..n {
            cnts.entry(nums[i]).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut tmp = cnts
            .iter()
            .map(|x| (*x.0, *x.1))
            .collect::<Vec<(i32, i32)>>();
        tmp.sort_by(|a, b| b.1.cmp(&a.1));
        let uk = k as usize;
        let mut res = vec![0; uk];
        for i in 0..uk as usize {
            res[i] = tmp[i].0;
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let res = Solution::top_k_frequent(nums, k);
    assert_eq!(res, vec![1, 2]);
}
