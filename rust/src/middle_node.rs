use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::find_list_length(head.as_ref());
        let mid = len / 2 + 1;
        Self::find_node(head, mid)
    }
    #[allow(clippy::borrowed_box)]
    fn find_list_length(head: Option<&Box<ListNode>>) -> u32 {
        match head {
            Some(node) => Self::find_list_length(node.next.as_ref()) + 1,
            None => 0,
        }
    }

    fn find_node(head: Option<Box<ListNode>>, index: u32) -> Option<Box<ListNode>> {
        match index {
            1 => head,
            0 => None,
            _ => Self::find_node(head.unwrap().next, index - 1),
        }
    }
}
