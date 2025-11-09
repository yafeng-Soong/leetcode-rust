use crate::solution::Solution;
use crate::utils::listnode::ListNode;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge(
            lists: &[Option<Box<ListNode>>],
            start: usize,
            end: usize,
        ) -> Option<Box<ListNode>> {
            let n = end - start;
            if n == 0 {
                return None;
            }

            if n == 1 {
                return lists[start].clone();
            }

            let mid = start + (end - start) / 2;
            let mut dummy = Box::new(ListNode::new(0));
            let mut left = merge(lists, start, mid);
            let mut right = merge(lists, mid, end);

            let mut cur = dummy.as_mut();
            while let (Some(l), Some(r)) = (&left, &right) {
                if l.val < r.val {
                    cur.next = left;
                    left = cur.next.as_mut().unwrap().next.take();
                } else {
                    cur.next = right;
                    right = cur.next.as_mut().unwrap().next.take();
                }

                cur = cur.next.as_mut().unwrap();
            }

            cur.next = if left.is_some() { left } else { right };

            dummy.next
        }

        merge(&lists, 0, lists.len())
    }
}

#[test]
fn test() {
    use crate::utils::listnode::build_listnode;

    struct Test {
        input: Vec<Vec<i32>>,
        expected: Option<Box<ListNode>>,
    }

    let tests = vec![
        Test {
            input: vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]],
            expected: build_listnode(vec![1, 1, 2, 3, 4, 4, 5, 6]),
        },
        Test {
            input: vec![],
            expected: build_listnode(vec![]),
        },
        Test {
            input: vec![vec![]],
            expected: build_listnode(vec![]),
        },
        Test {
            input: vec![vec![1], vec![2], vec![3]],
            expected: build_listnode(vec![1, 2, 3]),
        },
        Test {
            input: vec![vec![], vec![], vec![]],
            expected: build_listnode(vec![]),
        },
    ];

    for t in tests {
        let lists = t.input.into_iter().map(build_listnode).collect();
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result, t.expected);
    }
}
