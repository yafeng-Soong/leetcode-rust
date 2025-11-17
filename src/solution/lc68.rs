use crate::solution::Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut rows = Vec::new();
        let mut row_len = Vec::new();

        let n = words.len();
        let (mut left, mut right) = (0, 0);
        while left < n {
            let mut length = 0;
            while right < n && length + words[right].len() + right - left <= max_width as usize {
                length += words[right].len();
                right += 1;
            }

            row_len.push(length);
            let mut row = Vec::new();
            for w in words[left..right].iter() {
                row.push(w.clone());
            }

            rows.push(row);
            left = right;
        }

        let m = rows.len();
        rows.iter().take(m - 1).enumerate().for_each(|(i, row)| {
            let buckets = row.len() - 1;
            let mut sentence = String::new();

            match buckets {
                0 => {
                    sentence.push_str(&row[0]);
                    sentence.push_str(&" ".repeat(max_width as usize - row[0].len()));
                }
                _ => {
                    let remain = max_width as usize - row_len[i];
                    let mut gaps = vec![remain / buckets; buckets];
                    for gap in gaps[0..remain % buckets].iter_mut() {
                        *gap += 1;
                    }

                    for i in 0..buckets {
                        sentence.push_str(&row[i]);
                        sentence.push_str(&" ".repeat(gaps[i]));
                    }

                    sentence.push_str(&row[buckets]);
                }
            }

            res.push(sentence);
        });

        let last_row = rows[m - 1].join(" ");
        let last_len = last_row.len();
        let last_row = last_row + &" ".repeat(max_width as usize - last_len);
        res.push(last_row);

        res
    }
}

#[test]
fn test() {
    struct Test {
        input: Vec<String>,
        max_width: i32,
        expected: Vec<String>,
    }

    let tests = vec![
        Test {
            input: vec![
                "This".to_string(),
                "is".to_string(),
                "an".to_string(),
                "example".to_string(),
                "of".to_string(),
                "text".to_string(),
                "justification.".to_string(),
            ],
            max_width: 16,
            expected: vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string(),
            ],
        },
        Test {
            input: vec![
                "What".to_string(),
                "must".to_string(),
                "be".to_string(),
                "justified".to_string(),
            ],
            max_width: 16,
            expected: vec![
                "What   must   be".to_string(),
                "justified       ".to_string(),
            ],
        },
        Test {
            input: vec![
                "Science".to_string(),
                "is".to_string(),
                "what".to_string(),
                "we".to_string(),
                "understand".to_string(),
                "well".to_string(),
                "enough".to_string(),
                "to".to_string(),
                "explain".to_string(),
                "to".to_string(),
                "a".to_string(),
                "computer.".to_string(),
                "Art".to_string(),
                "is".to_string(),
                "everything".to_string(),
                "else".to_string(),
                "we".to_string(),
                "do".to_string(),
            ],
            max_width: 20,
            expected: vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string(),
            ],
        },
    ];

    for t in tests {
        assert_eq!(Solution::full_justify(t.input, t.max_width), t.expected);
    }
}
