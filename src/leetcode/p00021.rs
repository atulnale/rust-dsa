use crate::linked_list::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(n), None) | (None, Some(n)) => Some(n),
            (None, None) => None,
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    Some(Box::new(ListNode {
                        val: n1.val,
                        next: Self::merge_two_lists(n1.next, Some(n2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: n2.val,
                        next: Self::merge_two_lists(Some(n1), n2.next),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00021::Solution;
    use crate::linked_list::list_node::ListNode;

    #[test]
    fn test1() {
        let list1 = ListNode::build(vec![1,2,3]);
        let list2 = ListNode::build(vec![1,3,4]);
        let head = Solution::merge_two_lists(list1, list2);
        ListNode::display(&head);
    }
}
