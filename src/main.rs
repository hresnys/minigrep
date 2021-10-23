extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    //println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error! {}", e);
        process::exit(1);
    }

    // let mut f = File::open(config.filename).expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    // println!("With text:\n{}", contents);
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config {query, filename}
// }