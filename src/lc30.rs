use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = Vec::new();
        let size: usize = words.iter().map(|w| w.len()).sum();
        let (mut word_cnts, mut word_map) = (HashMap::new(), HashMap::new());
        for (_, word) in words.iter().enumerate() {
            word_cnts
                .entry(word.as_str())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        for i in 0..(s.len() - size + 1) {
            if i + size > s.len() {
                break;
            }

            let sub_s = &s[i..i + size];
            if word_map.contains_key(sub_s) {
                res.push(i as i32);
                continue;
            }

            if Self::is_sub(sub_s, &word_cnts) {
                res.push(i as i32);
                word_map.insert(sub_s, true);
            }
        }

        res
    }

    fn is_sub(s: &str, word_cnts: &HashMap<&str, i32>) -> bool {
        if s.len() == 0 {
            return true;
        }

        for (word, cnt) in word_cnts {
            if *cnt == 0 {
                continue;
            }

            if s.starts_with(word) {
                let mut new_cnts = word_cnts.clone();
                new_cnts.insert(&word, cnt - 1);
                if Self::is_sub(&s[word.len()..], &new_cnts) {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn test() {
    let res = Solution::find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string()],
    );
    assert_eq!(res, vec![0, 9]);
}
