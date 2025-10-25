use crate::{solution::Solution, utils::treenode::TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            None => None,
            Some(node) => {
                let left = Self::invert_tree(node.borrow().left.clone());
                let right = Self::invert_tree(node.borrow().right.clone());

                (node.borrow_mut().left, node.borrow_mut().right) = (right, left);
                root
            }
        }
    }

    pub fn invert_tree_loop(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root.clone()?;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(node);
        while !queue.is_empty() {
            let top = queue.pop_front().unwrap();
            let (left, right) = (top.borrow().left.clone(), top.borrow().right.clone());

            (top.borrow_mut().left, top.borrow_mut().right) = (right.clone(), left.clone());
            if let Some(left) = left {
                queue.push_back(left);
            }

            if let Some(right) = right {
                queue.push_back(right);
            }
        }

        root
    }
}

#[test]
fn test_invert_tree() {
    struct Test {
        root: Option<Rc<RefCell<TreeNode>>>,
        root_loop: Option<Rc<RefCell<TreeNode>>>,
        expected: Option<Rc<RefCell<TreeNode>>>,
    }

    let tests = vec![Test {
        root: TreeNode::from_vec(&[
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]),
        root_loop: TreeNode::from_vec(&[
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]),
        expected: TreeNode::from_vec(&[
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]),
    }];

    for t in tests {
        assert_eq!(Solution::invert_tree(t.root.clone()), t.expected);
        assert_eq!(Solution::invert_tree_loop(t.root_loop.clone()), t.expected);
    }
}
