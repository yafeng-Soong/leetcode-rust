use crate::{listnode::ListNode, solution::Solution};

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = dummy.as_mut();
        let (mut list1, mut list2) = (&list1, &list2);
        loop {
            match (list1, list2) {
                (None, None) => break,
                (None, Some(q)) => {
                    cur.next = Some(q.clone());
                    break;
                }
                (Some(p), None) => {
                    cur.next = Some(p.clone());
                    break;
                }
                (Some(p), Some(q)) => {
                    if p.val < q.val {
                        cur.next = Some(p.clone());
                        list1 = &p.next;
                    } else {
                        cur.next = Some(q.clone());
                        list2 = &q.next;
                    }
                    cur = cur.next.as_mut().unwrap();
                }
            };
        }

        dummy.next
    }
}

#[test]
fn test() {
    use crate::listnode::build_listnode;
    struct Test {
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
        res: Option<Box<ListNode>>,
    }
    let tests = vec![Test {
        list1: build_listnode(vec![1, 2, 4]),
        list2: build_listnode(vec![1, 3, 4]),
        res: build_listnode(vec![1, 1, 2, 3, 4, 4]),
    }];
    for test in tests {
        assert_eq!(Solution::merge_two_lists(test.list1, test.list2), test.res);
    }
}
