use crate::solution::Solution;

impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        // a * b * c = x -> x/b = a * c
        // input: [1,2,3,4], pre_prod: [1,2,6,24], suf_prod: [24,24,12,4]
        // output: [24,12,8,6]
        let mut res = nums.clone();
        let n = nums.len();
        let (mut pre_prod, mut suf_prod) = (1, 1);
        for i in 0..n {
            nums[i] *= pre_prod;
            res[n - i - 1] *= suf_prod;
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

#[test]
fn test_product_except_self() {
    struct Test {
        nums: Vec<i32>,
        expected: Vec<i32>,
    }

    let tests = vec![
        Test {
            nums: vec![1, 2, 3, 4],
            expected: vec![24, 12, 8, 6],
        },
        Test {
            nums: vec![-1, 1, 0, -3, 3],
            expected: vec![0, 0, 9, 0, 0],
        },
    ];

    for t in tests {
        assert_eq!(Solution::product_except_self(t.nums), t.expected);
    }
}
