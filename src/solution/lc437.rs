use crate::solution::Solution;
use crate::utils::treenode::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let target_sum = target_sum as i64;
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i64,
            sum: i64,
            sum_map: &mut HashMap<i64, i32>,
        ) -> i32 {
            let mut res = 0;
            if let Some(node) = root {
                let sum = sum + node.borrow().val as i64;
                let delta = sum - target_sum;
                if let Some(count) = sum_map.get(&delta) {
                    res += count;
                }

                sum_map.entry(sum).and_modify(|x| *x += 1).or_insert(1);
                res += dfs(node.borrow().left.clone(), target_sum, sum, sum_map);
                res += dfs(node.borrow().right.clone(), target_sum, sum, sum_map);
                sum_map.entry(sum).and_modify(|x| *x -= 1);
            }

            res
        }

        let mut sum_map = HashMap::from([(0, 1)]);
        dfs(root, target_sum, 0, &mut sum_map)
    }
}

#[test]
fn test() {
    struct Test {
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        expected: i32,
    }

    let tests = vec![
        Test {
            root: TreeNode::from_vec(&[
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                Some(5),
                Some(1),
            ]),
            target_sum: 22,
            expected: 3,
        },
        Test {
            root: TreeNode::from_vec(&[
                Some(10),
                Some(5),
                Some(-3),
                Some(3),
                Some(2),
                None,
                Some(11),
                Some(3),
                Some(-2),
                None,
                Some(1),
            ]),
            target_sum: 8,
            expected: 3,
        },
    ];

    for t in tests {
        assert_eq!(Solution::path_sum(t.root, t.target_sum), t.expected);
    }
}
