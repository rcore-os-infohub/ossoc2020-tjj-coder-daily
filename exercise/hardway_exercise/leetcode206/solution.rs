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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut v:Vec<Box<ListNode>>=vec![];
        while let Some(mut node)=head.take(){
            let next=node.next.take();
            v.push(node);
            head=next;
        }

        head=Some(Box::new(ListNode::new(0)));
        let mut p=&mut head;
  
        while !v.is_empty(){
            p.as_mut().unwrap().next=Some(v.pop().unwrap());
            p=&mut p.as_mut().unwrap().next;
        }
        head.unwrap().next
    }
}