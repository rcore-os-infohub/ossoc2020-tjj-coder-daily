use std::env;

fn can_print_it(c:char) -> bool{
    c.is_alphabetic() || c.is_ascii_whitespace()
}

fn print_letters(s:&str) {
    for c in s.chars(){
        if can_print_it(c){
            print!("'{}' == {} ", c, c as u8);
        }
    }
}

fn print_arguments(){
    for arg in env::args(){
        print_letters(&arg);
    }
}

fn main() {
    print_arguments();
}
