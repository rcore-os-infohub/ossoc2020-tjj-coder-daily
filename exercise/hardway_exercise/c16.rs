use std::fmt;

struct Person {
    name:String,
    age:usize,
    height:usize,
    weight:usize
}

impl Person{
    fn new(name:String,age:usize,height:usize,weight:usize) -> Box<Person>{
        Box::new(Person{
            name,age,height,weight
        })
    }

}

impl fmt::Display for Person{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{},{},{},{}",self.name,self.age,self.height,self.weight)
    }
}


fn main() {
    let person=Person::new(String::from("Tu"),20,175,70);
    println!("{}",person);
}
