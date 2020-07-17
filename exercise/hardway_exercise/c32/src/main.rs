use c32_linked_list::*;

fn main() {
    let mut list=LinkedList::from(vec![1,4,5]);
    list.push(88);
    let first=list.get_first_node().unwrap();
    let last=list.get_last_node().unwrap();
    println!("first={}",first.borrow().value);
    println!("last={}",last.borrow().value);
}