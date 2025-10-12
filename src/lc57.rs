mod lc57 {
    struct Solution;
    impl Solution {
        pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
            let mut res = Vec::new();
            let n = intervals.len();
            if n == 0 || intervals[n - 1][1] < new_interval[0] {
                res.extend(intervals);
                res.push(new_interval);
                return res;
            }

            let mut idx = 0;
            for i in 0..n {
                if intervals[i][1] < new_interval[0] {
                    res.push(intervals[i].clone());
                    idx = i + 1;
                    continue;
                }

                if intervals[i][0] > new_interval[1] {
                    res.push(new_interval);
                    res.extend(intervals[idx..].iter().cloned());
                    return res;
                }

                let lower = intervals[i][0].min(new_interval[0]);
                let upper = intervals[i][1].max(new_interval[1]);
                intervals[i] = vec![lower, upper];

                idx = i;
            }

            let mut right = idx;
            let mut upper = intervals[idx][1];
            while right < n && intervals[right][0] <= upper {
                upper = upper.max(intervals[right][1]);
                right += 1;
            }

            res.push(vec![intervals[idx][0], upper]);
            if right < n {
                res.extend(intervals[right..].iter().cloned());
            }

            res
        }
    }
}

#[test]
fn test() {}
