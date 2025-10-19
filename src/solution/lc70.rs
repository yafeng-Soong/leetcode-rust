use crate::solution::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 4 {
            return n;
        }

        let mut sum = vec![0; n as usize + 1];
        sum[1] = 1;
        sum[2] = 2;
        for i in 3..n + 1 {
            let idx = i as usize;
            sum[idx] = sum[idx - 1] + sum[idx - 2];
        }

        println!("{:?}", sum);
        sum[n as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(4), 5);
}
