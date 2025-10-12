use std::char;

use crate::solution::Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut words = vec!['a'];
        while words.len() < k as usize {
            Self::transform(&mut words);
        }

        words[k as usize - 1]
    }

    fn transform(words: &mut Vec<char>) {
        let chars = words.iter().map(|c| *c).collect::<Vec<_>>();
        for c in chars.iter() {
            let offset = (*c as u8 - 'a' as u8 + 1) % 26;
            let next = ('a' as u8 + offset) as char;
            words.push(next);
        }
    }
}
