use std::env;

fn main() {
    let args:Vec<String>=env::args().collect();
    if args.len() ==1 {
        panic!("You only have one argument. You suck.");
    }else if args.len()<4{
        println!("Here's your arguments:");
        for item in &args[1..]{
            print!("{} ",item);
        }
        println!();
    }else{
        panic!("You have too many arguments. You suck.");

    }
}
