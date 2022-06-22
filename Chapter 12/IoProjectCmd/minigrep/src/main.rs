extern crate minigrep;
use std::env; 
use std::process;

use minigrep::Config;

fn main() {
   
     //reading from the command line

    let args:Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = parse_configs(&args);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
    



}



fn parse_configs(args:&[String])->Config{
    
    Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Passing Argument: {}", err);
        process::exit(1);
    })
} 

