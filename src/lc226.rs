use crate::{solution::Solution, treenode::TreeNode};

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

    pub fn invert_tree2(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = match root.clone() {
            None => return None,
            Some(node) => node,
        };

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(node);
        while queue.len() > 0 {
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
