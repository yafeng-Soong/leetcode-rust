use crate::solution::Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnts = [0; 26];
        for c in magazine.chars() {
            cnts[c as usize - 'a' as usize] += 1;
        }

        for c in ransom_note.chars() {
            cnts[c as usize - 'a' as usize] -= 1;
            if cnts[c as usize - 'a' as usize] < 0 {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_can_construct() {
    struct Test {
        ransom_note: String,
        magazine: String,
        expected: bool,
    }

    let tests = vec![
        Test {
            ransom_note: "a".to_string(),
            magazine: "b".to_string(),
            expected: false,
        },
        Test {
            ransom_note: "aa".to_string(),
            magazine: "ab".to_string(),
            expected: false,
        },
        Test {
            ransom_note: "aa".to_string(),
            magazine: "aab".to_string(),
            expected: true,
        },
    ];

    for t in tests {
        assert_eq!(
            Solution::can_construct(t.ransom_note, t.magazine),
            t.expected
        );
    }
}
