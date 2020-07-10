use structopt_derive::*;

#[derive(StructOpt,Debug)]
#[structopt(name="csv_challenge",about="Usage")]
pub struct Opt{
    #[structopt(help="Input File")]
    pub input:String,
    #[structopt(help="Column Name")]
    pub column_name:String,
    #[structopt(help="Replacement Column Name")]
    pub replacement:String,
    #[structopt(help="Output File,stdout if not present")]
    pub output:Option<String>
}