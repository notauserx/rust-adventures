use std::env;
use std::fs;

fn main() {
    // collectiong the command line arguments in a vector
    // collect is one function we need to annotate types
    let args: Vec<String> = env::args().collect();
    
    //dbg!(args);

    let (query, file_path) = parse_config(&args);

    println!("Searching for -> {}", query);
    println!("In file -> {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Shoud have been able to read the file.");

    println!("With text:\n{contents}");

}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
