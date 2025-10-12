use crate::{solution::Solution, treenode::TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // clear but with more memory and cpu cast
        let n = preorder.len();
        if n == 0 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut idx = 0;
        while idx < n && inorder[idx] != preorder[0] {
            idx += 1;
        }

        root.borrow_mut().left =
            Self::build_tree(preorder[1..idx + 1].to_vec(), inorder[0..idx].to_vec());
        root.borrow_mut().right =
            Self::build_tree(preorder[idx + 1..n].to_vec(), inorder[idx + 1..n].to_vec());

        Some(root)
    }

    pub fn build_tree_without_clone(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();

        fn dfs(
            preorder: &[i32],
            left_1: usize,
            right_1: usize,
            inorder: &[i32],
            left_2: usize,
            right_2: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if left_1 >= right_1 {
                return None;
            }

            let root = Rc::new(RefCell::new(TreeNode::new(preorder[left_1])));
            let mut offset = 0;
            while left_2 + offset < right_2 && inorder[left_2 + offset] != preorder[left_1] {
                offset += 1;
            }

            root.borrow_mut().left = dfs(
                preorder,
                left_1 + 1,
                left_1 + offset + 1,
                inorder,
                left_2,
                left_2 + offset,
            );
            root.borrow_mut().right = dfs(
                preorder,
                left_1 + offset + 1,
                right_1,
                inorder,
                left_2 + offset + 1,
                right_2,
            );

            Some(root)
        }

        dfs(&preorder, 0, n, &inorder, 0, n)
    }
}

#[test]
fn test() {
    struct Test {
        preorder: Vec<i32>,
        inorder: Vec<i32>,
        expected: Option<Rc<RefCell<TreeNode>>>,
    }

    let tests = vec![
        Test {
            preorder: vec![3, 9, 20, 15, 7],
            inorder: vec![9, 3, 15, 20, 7],
            expected: TreeNode::from_vec(&[
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7),
            ]),
        },
        Test {
            preorder: vec![-1],
            inorder: vec![-1],
            expected: TreeNode::from_vec(&[Some(-1)]),
        },
    ];

    for t in tests {
        assert_eq!(
            Solution::build_tree(t.preorder.clone(), t.inorder.clone()),
            t.expected
        );
        assert_eq!(
            Solution::build_tree_without_clone(t.preorder, t.inorder),
            t.expected
        );
    }
}
