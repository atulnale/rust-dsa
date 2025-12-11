//! 2. Add Two Numbers
//!
//! URL: https://leetcode.com/problems/add-two-numbers/description/
//!
//! Submission: 100% runtime, 59.18% memory

use crate::linked_list::list_node::ListNode;

struct Solution {}
impl Solution {
    // pub fn add_two_numbers(
    //     mut l1: Option<Box<ListNode>>,
    //     mut l2: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     let mut result = ListNode {
    //         val: -1,
    //         next: None,
    //     };
    //     let mut ptr = &mut result;
    //     while l1.is_some() && l2.is_some() {
    //         let node1 = l1.unwrap();
    //         let node2 = l2.unwrap();
    //         match node1.val.cmp(&node2.val) {
    //             std::cmp::Ordering::Equal | std::cmp::Ordering::Less  => {
    //                 ptr.next = l1;
    //                 ptr = ptr.next.as_deref_mut().unwrap();
    //                 l1 = ptr.next.take();
    //             },
    //             std::cmp::Ordering::Greater => {
    //                 ptr.next = l2;
    //                 ptr = ptr.next.as_deref_mut().unwrap();
    //                 l2 = ptr.next.take();
    //             }
    //         }
    //     }
    //     if l1.is_none() {
    //         ptr.next = l1;
    //     }
    //     if l2.is_none() {
    //         ptr.next = l2;
    //     }
    //     result.next
    // }
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode {
            val: -1,
            next: None
        };
        let mut ptr = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let n1 = l1.as_ref().map_or(0, |node| node.val);
            let n2 = l2.as_ref().map_or(0, |node| node.val);
            let sum = n1 + n2 + carry;

            carry = sum / 10;
            ptr.next = Some(Box::new (ListNode {
                val: sum % 10,
                next: None
            }));
            ptr = ptr.next.as_deref_mut().unwrap();

            if let Some(node) = l1 {
                l1 = node.next;
            }
            if let Some(node) = l2 {
                l2 = node.next;
            }
        }
        head.next
    }

}

#[test]
fn test() {
    let head1 = ListNode::build(vec![1,2,3]);
    let head2 = ListNode::build(vec![1,2,3,4]);
    let res = Solution::add_two_numbers(head1, head2);
    ListNode::display(&res);
}
