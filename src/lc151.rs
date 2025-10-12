use crate::solution::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = s.split_whitespace().collect::<Vec<_>>();
        words.reverse();
        words.join(" ")
    }
}
