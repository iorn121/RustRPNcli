// use std::env;
use clap::{App,Arg};
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);
    let matches=App::new("My RPN program")
        .version("1.0.0")
        .author("Io")
        .about("My RPN program that calculates the command formula")
        .arg(
            Arg::new("formula_file")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required(false)
        )
        .get_matches();
    
    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified")
    }
    let verbose = matches.is_present("verbose");
    println!("Verbose is {}?", verbose);
}
