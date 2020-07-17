use std::env;

fn main() {
    let args:Vec<String>=env::args().collect();
    if args.len()!=2{
        panic!("ERROR: You need one argument.");
    }
    let s=&args[1];
    for c in s.chars(){
        match c {
            'a' | 'A' => println!("A"),
            'e' | 'E' => println!("E"),
            'i' | 'I' => println!("I"),
            'o' | 'O' => println!("O"),
            'u' | 'U' => println!("U"),
            'y' | 'Y' => println!("U"),
            _ => println!("{} is not a vowel",c),
        }
    }
}
