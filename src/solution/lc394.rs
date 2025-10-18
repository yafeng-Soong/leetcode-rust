use crate::solution::Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut res = String::new();
        let n = s.len();
        if n == 0 {
            return res;
        }

        let chars = s.chars().collect::<Vec<char>>();
        let (mut start, mut end) = (0, 0);
        while start < n {
            // add normal strs first.
            while end < n && chars[end].is_ascii_lowercase() {
                res.push(chars[end]);
                end += 1;
            }

            let mut times = 0_usize;
            while end < n && chars[end].is_digit(10) {
                times = times * 10 + chars[end].to_digit(10).unwrap() as usize;
                end += 1;
            }

            if times == 0 {
                break;
            }

            // mimic stack.
            let mut sum = 0;
            start = end + 1;
            while end < n {
                if chars[end] == '[' {
                    sum += 1;
                } else if chars[end] == ']' {
                    sum -= 1;
                }

                end += 1;

                if sum == 0 {
                    break;
                }
            }

            let sub = Self::decode_string(s.get(start..end).unwrap().to_string());
            res.push_str(&sub.repeat(times));

            start = end
        }

        res
    }
}
#[test]
fn test() {
    struct Test {
        input: String,
        res: String,
    }

    let tests = vec![
        Test {
            input: String::from("3[a]2[bc]"),
            res: String::from("aaabcbc"),
        },
        Test {
            input: String::from("3[a2[c]]"),
            res: String::from("accaccacc"),
        },
        Test {
            input: String::from("2[abc]3[cd]ef"),
            res: String::from("abcabccdcdcdef"),
        },
        Test {
            input: String::from("abc3[cd]xyz"),
            res: String::from("abccdcdcdxyz"),
        },
    ];

    for t in tests {
        assert_eq!(Solution::decode_string(t.input), t.res);
    }
}
