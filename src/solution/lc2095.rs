use crate::solution::Solution;
use crate::utils::listnode::ListNode;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let (mut fast, mut cnt) = (dummy.clone(), 0);
        while let Some(next) = fast.next {
            if let Some(next) = next.next {
                fast = next;
                cnt += 1;
            } else {
                break;
            }
        }

        let mut slow = dummy.as_mut();
        for _ in 0..cnt {
            slow = slow.next.as_mut().unwrap();
        }

        slow.next = slow.next.as_ref().unwrap().next.clone();
        dummy.next
    }
}

#[test]
fn test() {
    struct Test {
        head: Option<Box<ListNode>>,
        expected: Option<Box<ListNode>>,
    }

    let tests = vec![
        Test {
            head: ListNode::build_listnode(vec![1, 2, 3, 4, 5]),
            expected: ListNode::build_listnode(vec![1, 2, 4, 5]),
        },
        Test {
            head: ListNode::build_listnode(vec![1, 2, 3, 4]),
            expected: ListNode::build_listnode(vec![1, 2, 4]),
        },
        Test {
            head: ListNode::build_listnode(vec![1]),
            expected: ListNode::build_listnode(vec![]),
        },
    ];

    for t in tests {
        assert_eq!(Solution::delete_middle(t.head), t.expected);
    }
}
