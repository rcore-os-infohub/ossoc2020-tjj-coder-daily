use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::Debug;

pub struct Node<T:Debug>{
    pub value:T,
    pub next:Option<Rc<RefCell<Node<T>>>>,
    pub prev:Option<Weak<RefCell<Node<T>>>>
}

impl<T:Debug> Drop for Node<T>{
    fn drop(&mut self) {
        println!("drop Node value={:?}",self.value);
    }
}

impl<T:Debug> Node<T>{
    fn new(value:T) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node{
            value,
            next:None,
            prev:None
        }))
    }
}


pub struct LinkedList<T:std::fmt::Debug>{
    first:Option<Rc<RefCell<Node<T>>>>,
    last:Option<Rc<RefCell<Node<T>>>>
}

impl<T:Debug> From<Vec<T>> for LinkedList<T>{
    fn from(v: Vec<T>) -> Self {
        let mut list=LinkedList {
            first: None,
            last: None
        };
        let mut it=v.into_iter();

        let first_node=Node::new(it.next().unwrap());
        list.first=Some(Rc::clone(&first_node));

        let mut last_node=Rc::clone(&first_node);
        for item in it{
            let node=Node::new(item);
            (*last_node).borrow_mut().next=Some(Rc::clone(&node));
           // node.borrow_mut().prev=Some(Rc::downgrade(&last_node));
            last_node=Rc::clone(&node);
        }
        list.last=Some(Rc::clone(&last_node));
        list
    }

}

impl<T:Debug> LinkedList<T>{
    pub fn get_first_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
       match &self.first {
           Some(x) => Some(Rc::clone(x)),
           None => None
       }
    }
    pub fn get_last_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        match &self.last {
            Some(x) => Some(Rc::clone(x)),
            None => None
        }
    }

    pub fn push(& mut self,value:T){
        let node=Node::new(value);
        match & self.last {
            Some(x) =>{
                node.borrow_mut().prev=Some(Rc::downgrade(x));
                x.borrow_mut().next=Some(Rc::clone(&node));
                self.last=Some(Rc::clone(&node));
            },
            None => {
                self.last=Some(Rc::clone(&node));
                self.first=Some(Rc::clone(&node));
            }
        }
    }
}








