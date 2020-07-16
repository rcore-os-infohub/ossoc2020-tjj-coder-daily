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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut odd=Some(Box::new(ListNode::new(0)));
      let mut even=Some(Box::new(ListNode::new(0)));
      let mut oddp=&mut odd;
      let mut evenp=&mut even;

      let mut cnt=1;

      while head.is_some() {
        let mut this=head.take();
        head=this.as_mut().unwrap().next.take();
        this.as_mut().unwrap().next=None;

        if cnt%2!=0{
          oddp.as_mut().unwrap().next=this;
          oddp=& mut oddp.as_mut().unwrap().next;
        }else{
          evenp.as_mut().unwrap().next=this;
          evenp=& mut evenp.as_mut().unwrap().next;
        }
        cnt=cnt+1;
      }

      oddp.as_mut().unwrap().next=even.unwrap().next;


      odd.unwrap().next
    }
}