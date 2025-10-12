use crate::{listnode::ListNode, solution::Solution};
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = dummy.as_mut();
        let (mut l1, mut l2) = (l1, l2);
        loop {
            (l1, l2) = match (&mut l1, &mut l2) {
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
                    let sum = p.val + q.val;
                    p.as_mut().val = sum;
                    cur.next = Some(p.clone());
                    cur = cur.next.as_mut().unwrap();
                    (p.clone().next, q.clone().next)
                }
            };
        }

        let mut up = false;
        cur = dummy.as_mut();
        while let Some(p) = &mut cur.next {
            let mut sum = p.val;
            if up {
                sum += 1;
            }

            up = sum >= 10;
            p.val = sum % 10;
            cur = cur.next.as_mut().unwrap();
        }

        if up {
            cur.next = Some(Box::new(ListNode::new(1)));
        }

        dummy.next
    }
}

#[test]
fn test() {
    use crate::listnode::build_listnode;

    struct Test {
        l1: Vec<i32>,
        l2: Vec<i32>,
        expected: Vec<i32>,
    }
    let tests = vec![
        Test {
            l1: vec![2, 4, 3],
            l2: vec![5, 6, 4],
            expected: vec![7, 0, 8],
        },
        Test {
            l1: vec![0],
            l2: vec![0],
            expected: vec![0],
        },
        Test {
            l1: vec![9, 9, 9, 9, 9, 9, 9],
            l2: vec![9, 9, 9, 9],
            expected: vec![8, 9, 9, 9, 0, 0, 0, 1],
        },
        Test {
            l1: vec![6],
            l2: vec![8],
            expected: vec![4, 1],
        },
        Test {
            l1: vec![0],
            l2: vec![2, 7, 8],
            expected: vec![2, 7, 8],
        },
    ];
    for t in tests {
        let l1 = build_listnode(t.l1);
        let l2 = build_listnode(t.l2);
        let expected = build_listnode(t.expected);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
