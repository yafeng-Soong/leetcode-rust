use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let data: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let mut idx = 0;
        loop {
            let (mut map, mut over) = (HashMap::new(), false);
            for bs in data.iter() {
                match bs.get(idx) {
                    None => {
                        over = true;
                        break;
                    }
                    Some(b) => {
                        map.insert(*b, true);
                    }
                }
            }

            if over || map.len() > 1 {
                break;
            }

            idx += 1;
        }

        strs[0][..idx].to_string()
    }
}

#[test]
fn test() {
    let res = Solution::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    assert_eq!(res, "fl".to_string());
}
