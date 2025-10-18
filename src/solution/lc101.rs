use crate::{solution::Solution, utils::treenode::TreeNode};

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

#[test]
fn test() {
    struct Test {
        root: Option<Rc<RefCell<TreeNode>>>,
        expected: bool,
    }

    let tests = vec![
        Test {
            root: TreeNode::from_vec(&[
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(4),
                Some(4),
                Some(3),
            ]),
            expected: true,
        },
        Test {
            root: TreeNode::from_vec(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]),
            expected: false,
        },
    ];

    for t in tests {
        assert_eq!(Solution::is_symmetric(t.root), t.expected);
    }
}
