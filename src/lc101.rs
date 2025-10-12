use crate::{solution::Solution, treenode::TreeNode};

use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => Self::symmetric(root.borrow().left.clone(), root.borrow().right.clone()),
        }
    }

    fn symmetric(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let (left, right) = (left.borrow(), right.borrow());
                if left.val != right.val {
                    return false;
                }

                Self::symmetric(left.left.clone(), right.right.clone())
                    && Self::symmetric(left.right.clone(), right.left.clone())
            }
            _ => false,
        }
    }
}
