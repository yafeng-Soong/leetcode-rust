use crate::solution::Solution;
use crate::utils::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut max_val: i32) -> i32 {
            let mut res = 0;
            if let Some(node) = root {
                if node.borrow().val >= max_val {
                    res += 1;
                    max_val = node.borrow().val;
                }

                res += dfs(node.borrow_mut().left.take(), max_val);
                res += dfs(node.borrow_mut().right.take(), max_val);
            }

            res
        }

        dfs(root, i32::MIN)
    }
}

#[test]
fn test() {
    struct Test {
        root: Option<Rc<RefCell<TreeNode>>>,
        expected: i32,
    }

    let tests = vec![
        Test {
            root: TreeNode::from_vec(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)]),
            expected: 4,
        },
        Test {
            root: TreeNode::from_vec(&[Some(3), Some(3), None, Some(4), Some(2)]),
            expected: 3,
        },
        Test {
            root: TreeNode::from_vec(&[Some(1)]),
            expected: 1,
        },
    ];

    for t in tests {
        assert_eq!(Solution::good_nodes(t.root), t.expected);
    }
}
