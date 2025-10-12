use crate::{solution::Solution, treenode::TreeNode};

use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (None, None) => true,
            (Some(left), Some(right)) => {
                let (left, right) = (left.borrow(), right.borrow());
                if left.val != right.val {
                    return false;
                }

                Self::is_same_tree(left.left.clone(), right.left.clone())
                    && Self::is_same_tree(left.right.clone(), right.right.clone())
            }
        }
    }
}

#[test]
fn test() {
    struct Test {
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
        res: bool,
    }
    let tests = vec![
        Test {
            p: TreeNode::from_vec(&[Some(1), Some(2), Some(3)]),
            q: TreeNode::from_vec(&[Some(1), Some(2), Some(3)]),
            res: true,
        },
        Test {
            p: TreeNode::from_vec(&[Some(1), Some(2)]),
            q: TreeNode::from_vec(&[Some(1), None, Some(2)]),
            res: false,
        },
        Test {
            p: TreeNode::from_vec(&[Some(1), Some(2), Some(1)]),
            q: TreeNode::from_vec(&[Some(1), Some(1), Some(2)]),
            res: false,
        },
    ];

    for test in tests {
        let res = Solution::is_same_tree(test.p, test.q);
        assert_eq!(res, test.res);
    }
}
