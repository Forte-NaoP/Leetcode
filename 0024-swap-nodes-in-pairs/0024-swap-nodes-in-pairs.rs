// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        if head.is_none() {
            return head;
        }

        let mut v = vec![];
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.clone();
            node.next = None;
            v.push(node);

        }

        if v.len() < 2 {
            head = Some(v[0].clone());
            return head;
        }

        let len = if v.len() % 2 == 1 {
            v.len()-1
        } else {
            v.len()
        };

        for i in (0..len).step_by(2) {
            v.swap(i, i+1);
        }

        for i in (1..=v.len()-1).rev() {
            v[i-1].next = Some(v[i].clone());
        }

        Some(v[0].clone())

    }
}