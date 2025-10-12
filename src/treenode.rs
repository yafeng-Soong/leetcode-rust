use std::{cell::RefCell, rc::Rc};
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(v: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let n = v.len();
        if n == 0 {
            return None;
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone().unwrap());
        let mut i = 1;
        while queue.len() > 0 && i < n {
            let node = queue.pop_front().unwrap();
            if let Some(val) = v[i] {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().left.clone().unwrap());
            }
            i += 1;

            if i >= n {
                break;
            }

            if let Some(val) = v[i] {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().right.clone().unwrap());
            }
            i += 1;
        }

        root
    }
}
