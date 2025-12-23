use crate::linked_list::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = ListNode {
            val: -1,
            next: None,
        };
        let mut curr = &mut head;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let num1 = l1.as_ref().map_or(0, |node| node.val);
            let num2 = l2.as_ref().map_or(0, |node| node.val);

            let sum = carry + num1 + num2;
            let n = Some(Box::new(ListNode {
                val: sum % 10,
                next: None,
            }));
            curr.next = n;
            carry = sum / 10;
            curr = curr.next.as_mut().unwrap();
            if let Some(node) = l1 {
                l1 = node.next;
            }
            if let Some(node) = l2 {
                l2 = node.next;
            }
        }
        return head.next;
    }
}
