use crate::solution::Solution;
use crate::utils::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q {
            return root;
        }

        match root.clone() {
            None => None,
            Some(node) => {
                let left =
                    Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
                let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p, q);
                match (left.clone(), right.clone()) {
                    (Some(_), Some(_)) => root,
                    (Some(_), None) => left,
                    (None, Some(_)) => right,
                    (None, None) => None,
                }
            }
        }
    }
}
