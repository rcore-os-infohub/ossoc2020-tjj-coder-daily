// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

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

    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

      let mut list=Some(Box::new(ListNode::new(0)));
      let mut p=&mut list;

      let mut x=0;//进位

      loop {
        match (l1.as_mut(),l2.as_mut()) {
          (None,None)=>{break;},
          (Some(node),None)=>{
            let mut val=node.val+x;
            x=0;
            if val>=10{
              x=val/10;
              val=val%10;
            }
            let mut next=Some(Box::new(ListNode::new(val)));
            p.as_mut().unwrap().next=next;
            p=&mut p.as_mut().unwrap().next;
            l1=l1.unwrap().next;
          },
          (None,Some(node))=>{
            let mut val=node.val+x;
            x=0;
            if val>=10{
              x=val/10;
              val=val%10;
            }
            let mut next=Some(Box::new(ListNode::new(val)));
            p.as_mut().unwrap().next=next;
            p=&mut p.as_mut().unwrap().next;
            l2=l2.unwrap().next;
          },
          (Some(node1),Some(node2))=>{
            let mut val=node1.val+node2.val+x;
            x=0;
            if val>=10{
              x=val/10;
              val=val%10;
            }
            let mut next=Some(Box::new(ListNode::new(val)));
            p.as_mut().unwrap().next=next;
            p=&mut p.as_mut().unwrap().next;
            l1=l1.unwrap().next;
            l2=l2.unwrap().next;
          }
        }
      }
      if x!=0{
        let mut next=Some(Box::new(ListNode::new(x)));
        p.as_mut().unwrap().next=next;
      }


      list.unwrap().next
    }
}