use crate::{solution::Solution, treenode::TreeNode};

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
