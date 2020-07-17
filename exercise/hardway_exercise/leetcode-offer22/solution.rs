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
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
     let (mut fast_p,mut slow_p)=(&head,&head);
     for _i in 1..k+1{
         fast_p=&fast_p.as_ref().unwrap().next;
     }

     while fast_p.is_some() {
         fast_p=&fast_p.as_ref().unwrap().next;
         slow_p=&slow_p.as_ref().unwrap().next;
     }

     slow_p.clone()
 }
}