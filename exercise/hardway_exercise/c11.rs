use std::env;

fn main() {
   for arg in env::args(){
       println!("{}",arg);
   }

    let strings=[String::from("abc"),String::from("sdsd"),String::from("sd")];
    let mut i:usize=0;
    while i<strings.len() {
        println!("{}",strings[i]);
        i=i+1;
    }
}
