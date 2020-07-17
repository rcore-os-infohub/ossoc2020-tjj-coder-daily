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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let (mut ans,mut queue)=(vec![],vec![]);
        if let Some(rc_node)=root{
            queue.push(Rc::clone(&rc_node));
            while !queue.is_empty() {
                let rc_node=queue.remove(0);
                let node=rc_node.borrow();
                ans.push(node.val);
                if let Some(rc_left_node)=node.left.clone(){
                    queue.push(Rc::clone(&rc_left_node));
                }
                if let Some(rc_right_node)=node.right.clone(){
                    queue.push(Rc::clone(&rc_right_node));
                }
            }
        }

        ans
    }
}