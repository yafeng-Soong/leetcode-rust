use crate::solution::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut cnt, mut cur) = (1, nums[0]);
        for num in nums {
            match cnt {
                0 => {
                    cur = num;
                    cnt = 1;
                }
                _ => {
                    if cur == num {
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

#[test]
fn test() {
    struct Test {
        nums: Vec<i32>,
        expected: i32,
    }

    let tests = vec![
        Test {
            nums: vec![3, 2, 3],
            expected: 3,
        },
        Test {
            nums: vec![2, 2, 1, 1, 1, 2, 2],
            expected: 2,
        },
    ];

    for t in tests {
        assert_eq!(Solution::majority_element(t.nums), t.expected);
    }
}
