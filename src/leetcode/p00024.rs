use crate::linked_list::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut p1 = dummy.as_mut();
        while p1.next.is_some() && p1.next.as_ref().unwrap().next.is_some() {
            let mut p2 = p1.next.take().unwrap();
            let mut p3 = p2.next.take().unwrap();
            p2.next = p3.next.take();
            p3.next = Some(p2);
            p1.next = Some(p3);
            p1 = p1.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00024::Solution;
    use crate::linked_list::list_node::ListNode;

    #[test]
    fn test1() {
        let list1 = ListNode::build(vec![1, 2, 3, 4]);
        let result = Solution::swap_pairs(list1);
        ListNode::display(&result);
    }
}
