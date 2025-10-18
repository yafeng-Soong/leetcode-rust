use crate::{listnode::ListNode, solution::Solution};
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));
        let mut next = head;
        while let Some(mut p) = next {
            next = p.next;
            p.next = root.next;
            root.next = Some(p);
        }

        root.next
    }
}

#[test]
fn test() {
    use crate::listnode::build_listnode;

    let mut head = build_listnode(vec![1, 2, 3, 4, 5]);
    let mut want = build_listnode(vec![5, 4, 3, 2, 1]);
    let mut res = Solution::reverse_list(head);
    assert_eq!(res, want);

    head = build_listnode(vec![1, 2]);
    want = build_listnode(vec![2, 1]);
    res = Solution::reverse_list(head);
    assert_eq!(res, want);

    head = build_listnode(vec![]);
    want = build_listnode(vec![]);
    res = Solution::reverse_list(head);
    assert_eq!(res, want);
}
