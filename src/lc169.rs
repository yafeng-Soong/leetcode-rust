use crate::solution::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut cnt, mut cur) = (1, nums[0]);
        let n = nums.len();
        for i in 1..n {
            match cnt {
                0 => {
                    cur = nums[i];
                    cnt = 1;
                }
                _ => {
                    if cur == nums[i] {
                        cnt += 1;
                    } else {
                        cnt -= 1;
                    }
                }
            }
        }

        cur
    }
}
