use regex::Regex;
use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        let biz_set = HashMap::from([
            ("electronics".to_string(), 0),
            ("grocery".to_string(), 1),
            ("pharmacy".to_string(), 2),
            ("restaurant".to_string(), 3),
        ]);

        let mut res = code
            .into_iter()
            .enumerate()
            .filter(|(i, c)| {
                re.is_match(c)
                    && biz_set.contains_key(business_line.get(*i).unwrap())
                    && is_active[*i]
            })
            .collect::<Vec<_>>();
        res.sort_by(|a, b| {
            biz_set
                .get(business_line.get(a.0).unwrap())
                .unwrap()
                .cmp(biz_set.get(business_line.get(b.0).unwrap()).unwrap())
                .then(a.1.cmp(&b.1))
        });
        res.into_iter().map(|(_, c)| c.clone()).collect()
    }
}

#[test]
fn test() {
    struct Test {
        code: Vec<&'static str>,
        business_line: Vec<&'static str>,
        is_active: Vec<bool>,
        expected: Vec<String>,
    }

    let tests = vec![
        Test {
            code: vec!["SAVE20", "", "PHARMA5", "SAVE@20"],
            business_line: vec!["restaurant", "grocery", "pharmacy", "restaurant"],
            is_active: vec![true, true, true, true],
            expected: vec!["PHARMA5".to_string(), "SAVE20".to_string()],
        },
        Test {
            code: vec!["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"],
            business_line: vec!["grocery", "electronics", "invalid"],
            is_active: vec![false, true, true],
            expected: vec!["ELECTRONICS_50".to_string()],
        },
    ];

    for t in tests {
        assert_eq!(
            Solution::validate_coupons(
                t.code.into_iter().map(|s| s.to_string()).collect(),
                t.business_line.into_iter().map(|s| s.to_string()).collect(),
                t.is_active
            ),
            t.expected
        );
    }
}
