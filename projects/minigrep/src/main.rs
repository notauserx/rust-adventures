use minigrep::Config;
use std::env;
use std::process;

fn main() {
  // collectiong the command line arguments in a vector
  // collect is one function we need to annotate types
  let args: Vec<String> = env::args().collect();

  //dbg!(args);

  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  //println!("Searching for -> {}", config.query);
  //println!("In file -> {}", config.file_path);

  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {e}");
    process::exit(1);
  }
}
