use std::{cmp::Reverse, collections::BinaryHeap};

use crate::linked_list::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        let mut ans = ListNode {
            val: -1,
            next: None,
        };
        let mut curr_ptr = &mut ans;
        for ele in lists {
            if ele.is_some() {
                min_heap.push(ele.unwrap());
            }
        }

        while let Some(mut node) = min_heap.pop() {
            let temp = node.next.take();
            curr_ptr.next = Some(node);
            curr_ptr = curr_ptr.next.as_deref_mut().unwrap();
            if let Some(n1) = temp {
                min_heap.push(n1);
            }
        }
        ans.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00023::Solution;
    use crate::linked_list::list_node::ListNode;

    #[test]
    fn test1() {
        let list1 = ListNode::build(vec![1, 2, 3]);
        let list2 = ListNode::build(vec![1, 3, 4]);
        let res = Solution::merge_k_lists(vec![list1, list2]);

        ListNode::display(&res);
    }
}
