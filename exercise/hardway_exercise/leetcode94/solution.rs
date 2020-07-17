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
    pub fn inorder_traversal_res(root: &Option<Rc<RefCell<TreeNode>>>,v:&mut Vec<i32>){
       if let Some(ref_node)=root{
           let node=ref_node.borrow();
           Solution::inorder_traversal_res(&node.left,v);
           v.push(node.val);
           Solution::inorder_traversal_res(&node.right,v);
       }

    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans:Vec<i32>=vec![];
        Solution::inorder_traversal_res(&root,&mut ans);
        ans
    }
}