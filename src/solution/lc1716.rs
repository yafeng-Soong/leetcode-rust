use crate::solution::Solution;
impl Solution {
    const SUM_OF_WEEK: i32 = 28;
    pub fn total_money(n: i32) -> i32 {
        let mut res = 0;
        let num_of_week = n / 7;
        let days = n % 7;
        res += Self::SUM_OF_WEEK * num_of_week + num_of_week * (num_of_week - 1) / 2 * 7;
        res += num_of_week * days + days * (days + 1) / 2;
        res
    }
}

#[test]
fn test() {
    struct Test {
        n: i32,
        expected: i32,
    }

    let tests = vec![
        Test { n: 4, expected: 10 },
        Test {
            n: 10,
            expected: 37,
        },
        Test {
            n: 20,
            expected: 96,
        },
    ];

    for t in tests {
        assert_eq!(Solution::total_money(t.n), t.expected);
    }
}
