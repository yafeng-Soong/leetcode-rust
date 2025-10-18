use crate::solution::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = s.split_whitespace().collect::<Vec<_>>();
        words.reverse();
        words.join(" ")
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
            s: "the sky is blue".to_string(),
            expected: "blue is sky the".to_string(),
        },
        Test {
            s: "  hello world!  ".to_string(),
            expected: "world! hello".to_string(),
        },
    ];

    for t in tests {
        assert_eq!(Solution::reverse_words(t.s), t.expected);
    }
}
