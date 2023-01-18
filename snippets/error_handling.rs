use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
  //match_on_different_errors();
  //open_with_unwrap();
  // in professional code, Rustaceans prefer expect instead of unwrap
  //open_with_expect();

  /*
  let username_result = read_username_from_file();

  match username_result {
    Ok(username) => println!("{username}"),
    Err(_) => println!("Oops"),
  }

  */

  //let username_result = read_username_from_file_compact().unwrap();
  let username_result = read_username_from_file_compact()?;

  println!("{username_result}");

  Ok(())
}

fn handle_result_variants() {
  let file_result = File::open("hello.txt");
 
  let file = match file_result {
    Ok(file)    => file,
    Err(error)  => panic!("Problem opening the file: {:?}", error),
  };
}

fn match_on_different_errors() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error);
      }
    },
  };
}

fn open_with_unwrap() {
  let greeting_file = File::open("hello.txt").unwrap();

}

fn open_with_expect() {
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt was not found.");

}

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("error_handling_username_file.txt");

  let mut username_file = match username_file_result {
    Ok(file)  => file,
    Err(e)    => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}

fn read_username_from_file_compact() -> Result<String, io::Error> {
  //let mut username_file = File::open("error_handling_username_file.txt")?;
  let mut username = String::new();

  //username_file.read_to_string(&mut username);
  File::open("error_handling_username_file.txt")?.read_to_string(&mut username);
  //Ok(username)

  fs::read_to_string("error_handling_username_file.txt")
}