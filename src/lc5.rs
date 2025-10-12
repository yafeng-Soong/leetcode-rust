use crate::solution::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut start, mut end) = (0, 0);
        let n = s.len();
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..(2 * n - 1) {
            let (mut left, mut right) = ((i / 2) as i32, (i + 1) / 2);
            while left >= 0 && right < n && chars[left as usize] == chars[right] {
                if right - left as usize + 1 > end - start + 1 {
                    (start, end) = (left as usize, right);
                }
                left -= 1;
                right += 1;
            }
        }

        s.get(start..=end).unwrap().to_string()
    }
}

#[test]
fn test() {
    struct Test {
        s: String,
        expected: String,
    }

    let tests = vec![
        Test {
            s: "babad".to_string(),
            expected: "bab".to_string(),
        },
        Test {
            s: "cbbd".to_string(),
            expected: "bb".to_string(),
        },
    ];

    for t in tests {
        assert_eq!(Solution::longest_palindrome(t.s), t.expected);
    }
}
