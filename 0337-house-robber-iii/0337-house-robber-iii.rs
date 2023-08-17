// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        subtree_max(root).0
    }
}

fn subtree_max(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {

    use std::cmp::max;

    let mut val = 0;
    let mut last_val = 0;
    if let Some(node) = root {
        let node = node.borrow();
        let left_max = subtree_max(node.left.clone());
        let right_max = subtree_max(node.right.clone());
        val = max(left_max.0+right_max.0, left_max.1+right_max.1+node.val);
        last_val = left_max.0+right_max.0;
    }
    (val, last_val)
}