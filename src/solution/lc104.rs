use crate::{solution::Solution, utils::treenode::TreeNode};

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_depth = Self::max_depth(node.borrow().left.clone());
                let right_depth = Self::max_depth(node.borrow().right.clone());
                max(left_depth, right_depth) + 1
            }
        }
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
            root: TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]),
            expected: 3,
        },
        Test {
            root: TreeNode::from_vec(&[Some(1), None, Some(2)]),
            expected: 2,
        },
    ];

    for t in tests {
        assert_eq!(Solution::max_depth(t.root), t.expected);
    }
}
