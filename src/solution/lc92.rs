use crate::{solution::Solution, utils::listnode::ListNode};

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut start = dummy.as_mut();
        for _ in 1..left {
            start = start.next.as_mut().unwrap();
        }

        let mut cur = start.next.take();
        let mut p = cur.as_mut().unwrap();
        for _ in left..right {
            p = p.next.as_mut().unwrap();
        }

        start.next = p.next.take();

        let mut q;
        for _ in left..=right {
            q = cur.clone().unwrap().next;
            cur.as_mut().unwrap().next = start.next.clone();
            start.next = cur;
            cur = q;
        }

        dummy.next
    }
}

#[test]
fn test() {
    use crate::utils::listnode::build_listnode;
    struct Test {
        head: Vec<i32>,
        left: i32,
        right: i32,
        expected: Vec<i32>,
    }
    let tests = vec![Test {
        head: vec![1, 2, 3, 4, 5],
        left: 2,
        right: 4,
        expected: vec![1, 4, 3, 2, 5],
    }];
    for test in tests {
        let head = build_listnode(test.head);
        let expected = build_listnode(test.expected);
        let actual = Solution::reverse_between(head, test.left, test.right);
        assert_eq!(actual, expected);
    }
}
