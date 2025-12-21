use crate::solution::Solution;
use crate::utils::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn gen_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut res = vec![];
            if let Some(node) = root {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if left.is_none() && right.is_none() {
                    return vec![node.borrow().val];
                }

                if left.is_some() {
                    res.append(&mut gen_leaves(left));
                }

                if right.is_some() {
                    res.append(&mut gen_leaves(right));
                }
            }

            res
        }

        let leaves1 = gen_leaves(root1);
        let leaves2 = gen_leaves(root2);
        leaves1 == leaves2
    }
}

#[test]
fn test() {
    struct Test {
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
        expected: bool,
    }

    let test_cases = vec![
        Test {
            root1: TreeNode::from_vec(&[
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(9),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ]),
            root2: TreeNode::from_vec(&[
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(7),
                Some(4),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(9),
                Some(8),
            ]),
            expected: true,
        },
        Test {
            root1: TreeNode::from_vec(&[Some(1), Some(2), Some(3)]),
            root2: TreeNode::from_vec(&[Some(1), Some(3), Some(2)]),
            expected: false,
        },
    ];

    for t in test_cases {
        assert_eq!(Solution::leaf_similar(t.root1, t.root2), t.expected);
    }
}
