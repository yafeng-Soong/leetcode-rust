use crate::solution::Solution;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut nums: Vec<i32> = (lo..=hi).collect();
        nums.sort_by(|&a, &b| {
            let res = Self::compute_weight(a).cmp(&Self::compute_weight(b));
            match res {
                std::cmp::Ordering::Equal => a.cmp(&b),
                _ => res,
            }
        });

        nums[k as usize - 1]
    }

    fn compute_weight(mut num: i32) -> i32 {
        let mut res = 0;
        loop {
            match num {
                1 => break,
                n if n % 2 == 0 => num /= 2,
                _ => num = 3 * num + 1,
            }
            res += 1;
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        lo: i32,
        hi: i32,
        k: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            lo: 12,
            hi: 15,
            k: 2,
            expected: 13,
        },
        Test {
            lo: 7,
            hi: 11,
            k: 4,
            expected: 7,
        },
    ];

    for t in tests {
        let res = Solution::get_kth(t.lo, t.hi, t.k);
        assert_eq!(t.expected, res);
    }
}
