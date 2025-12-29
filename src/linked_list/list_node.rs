#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) // use Ord's cmp
    }
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn build(arr: Vec<i32>) -> Option<Box<Self>> {
        let mut dummy = ListNode {
            val: -1,
            next: None,
        };
        let mut ptr = &mut dummy;
        for ele in arr {
            ptr.next = Some(Box::new(ListNode {
                val: ele,
                next: None,
            }));
            ptr = ptr.next.as_mut().unwrap();
        }
        dummy.next
    }

    pub fn display(mut head: &Option<Box<Self>>) {
        let mut curr = head.as_deref();
        while let Some(node) = curr {
            print!(" -> {}", node.val);
            curr = node.next.as_deref();
        }
        println!();
    }
}
