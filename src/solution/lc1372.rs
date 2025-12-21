use crate::solution::Solution;
use crate::utils::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32, len: i32, is_left: bool) {
            *res = (*res).max(len);
            if let Some(node) = root {
                if is_left {
                    dfs(node.borrow().right.clone(), res, len + 1, false);
                    dfs(node.borrow().left.clone(), res, 0, true);
                } else {
                    dfs(node.borrow().left.clone(), res, len + 1, true);
                    dfs(node.borrow().right.clone(), res, 0, false);
                }
            }
        }

        match root {
            None => unreachable!(),
            Some(node) => {
                dfs(node.borrow().left.clone(), &mut res, 0, true);
                dfs(node.borrow().right.clone(), &mut res, 0, false);
            }
        }

        res
    }
}

#[test]
fn test() {
    struct Test {
        input: Option<Rc<RefCell<TreeNode>>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            input: TreeNode::from_vec(&[
                Some(1),
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                Some(1),
                Some(1),
                None,
                Some(1),
                None,
                None,
                None,
                Some(1),
                None,
                Some(1),
            ]),
            expected: 3,
        },
        Test {
            input: TreeNode::from_vec(&[
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(1),
                None,
                None,
                Some(1),
                Some(1),
                None,
                Some(1),
            ]),
            expected: 4,
        },
        Test {
            input: TreeNode::from_vec(&[Some(1)]),
            expected: 0,
        },
    ];

    for t in tests {
        assert_eq!(Solution::longest_zig_zag(t.input), t.expected);
    }
}
