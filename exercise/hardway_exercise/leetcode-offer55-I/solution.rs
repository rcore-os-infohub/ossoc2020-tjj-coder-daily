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
    
    fn max_depth_help(root:&Option<Rc<RefCell<TreeNode>>>) -> i32{
        if let Some(rc_node)=root{
            let node=rc_node.borrow();
            let left_depth=Solution::max_depth_help(&node.left);
            let right_depth=Solution::max_depth_help(&node.right);
            std::cmp::max(left_depth,right_depth)+1
        }else{
            0
        }

    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_help(&root)

    }
}