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
    fn reverse_print_help(head: &Option<Box<ListNode>>,ans:&mut Vec<i32>){
        if let Some(node)=head{
            Solution::reverse_print_help(&node.next,ans);
            ans.push(node.val);
        }
    }

    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans=vec![];
        Solution::reverse_print_help(&head,&mut ans);
        ans
    }
}