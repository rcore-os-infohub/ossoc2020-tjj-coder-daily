mod opt;

use csv_challenge::{load_csv, replace_column, write_csv};
use opt::Opt;
use structopt::StructOpt;
use std::path::PathBuf;
use std::process;

fn main() {

    let opt=Opt::from_args();
    let input=PathBuf::from(opt.input);
    let output_filename=opt.output.unwrap_or(r"src\output\output.csv".to_string());

    let content=match load_csv(input){
        Ok(content) => content,
        Err(_) => process::exit(1)
    };

    let result=match replace_column(content,&opt.column_name,&opt.replacement) {
        Ok(s) => s,
        Err(_) => process::exit(2)
    };

    match write_csv(&result,&output_filename){
        Err(_) => process::exit(3),
        _ => ()
    }


}