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
        let chars = words.to_vec();
        for c in chars.iter() {
            let offset = (*c as u8 - b'a' + 1) % 26;
            let next = (b'a' + offset) as char;
            words.push(next);
        }
    }
}

#[test]
fn test_kth_character() {
    struct Test {
        k: i32,
        expected: char,
    }

    let tests = vec![
        Test {
            k: 5,
            expected: 'b',
        },
        Test {
            k: 10,
            expected: 'c',
        },
    ];

    for t in tests {
        assert_eq!(Solution::kth_character(t.k), t.expected);
    }
}
