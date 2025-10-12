mod lc228 {
    pub struct Solution;
    impl Solution {
        pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
            let mut res = Vec::new();
            let mut left = 0;
            let mut right = 0;
            let n = nums.len();
            while left < n {
                while right < n && nums[right] - nums[left] == (right - left) as i32 {
                    right += 1;
                }

                res.push(if right - left == 1 {
                    nums[left].to_string()
                } else {
                    format!("{}->{}", nums[left], nums[right - 1])
                });
                left = right;
            }

            res
        }
    }
}

#[test]
fn test() {
    struct Test {
        nums: Vec<i32>,
        expect: Vec<String>,
    }
    let tests = vec![
        Test {
            nums: vec![0, 1, 2, 4, 5, 7],
            expect: vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()],
        },
        Test {
            nums: vec![0, 2, 3, 4, 6, 8, 9],
            expect: vec![
                "0".to_string(),
                "2->4".to_string(),
                "6".to_string(),
                "8->9".to_string(),
            ],
        },
    ];
    for test in tests {
        assert_eq!(lc228::Solution::summary_ranges(test.nums), test.expect);
    }
}
