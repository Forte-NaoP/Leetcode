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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = Box::new(ListNode::new(0));
        let mut current = &mut ans;
        let mut list1 = list1;
        let mut list2 = list2;
        
        while let (Some(node1), Some(node2)) = (&mut list1, &mut list2) {
            if node1.val < node2.val {
                current.next = list1.take();
                list1 = current.next.as_mut().unwrap().next.take();
            } else {
                current.next = list2.take();
                list2 = current.next.as_mut().unwrap().next.take();
            }
            current = current.next.as_mut().unwrap();
        }

        if list1.is_some() {
            current.next = list1;
        } else if list2.is_some() {
            current.next = list2;
        }

        ans.next
    }
}