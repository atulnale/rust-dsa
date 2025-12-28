use crate::linked_list::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut front = dummy.as_mut() as *mut ListNode;
        let mut back = dummy.as_mut() as *mut ListNode;
        unsafe {
            while n > 0 {
                front = (*front).next.as_deref_mut().unwrap() as *mut ListNode;
                n -= 1;
            }
            while (*front).next.is_some() {
                front = (*front).next.as_deref_mut().unwrap() as *mut ListNode;
                back = (*back).next.as_deref_mut().unwrap() as *mut ListNode;
            }
            let node_to_remove = (*back).next.take().unwrap();
            (*back).next = node_to_remove.next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00019::Solution;
    use crate::linked_list::list_node::ListNode;

    #[test]
    fn test1() {
        let head = ListNode::build(vec![1, 2, 3, 4, 5]);
        let ans = Solution::remove_nth_from_end(head, 2);
        ListNode::display(&ans);
    }
}
