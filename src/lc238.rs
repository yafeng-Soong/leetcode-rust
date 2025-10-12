use crate::solution::Solution;

impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        // a * b * c = x -> x/b = a * c
        // [1,2,6,24] [24,24,12,4]
        let mut res = nums.clone();
        let n = nums.len();
        let (mut pre_prod, mut suf_prod) = (1, 1);
        for i in 0..n {
            nums[i] = pre_prod * nums[i];
            res[n - i - 1] = suf_prod * res[n - i - 1];
            pre_prod = nums[i];
            suf_prod = res[n - i - 1];
        }

        for i in 0..n {
            match i {
                0 => res[i] = res[i + 1],
                val if val == n - 1 => res[i] = nums[i - 1],
                _ => res[i] = nums[i - 1] * res[i + 1],
            }
        }

        res
    }
}
