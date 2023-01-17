// cargo script strings.rs from directory
fn main() {
  // creating a new string
  //let mut s = String::new();

  let data = "initial contents";

  let s = data.to_string();

  println!("{s}");

  let s = String::from("another content");

  println!("{s}");

  // updating a string

  println!("updating a string");

  let mut s1 = String::from("hello");

  // push_str takes a string slice
  // we don't want to take ownership of the parameter
  let s2 = "/ peeps";
  s1.push_str(s2);

  //
  println!("s1 is {s1}");
  println!("s2 is {s2}");

  // push appends a single characters
  let mut s = String::from("bo");
  s.push('b');

  println!("s is {s} after push");

  println!("string concatenation");

  let s1 = String::from("a");
  let s2 = String::from("b");
  let s3 = String::from("c");

  //let s = s1 + "-" + &s2 + "-" + &s3;
  let s = format!("{s1}.{s2}.{s3}");

  println!("{s}");

  println!("Indexing into strings...");

  // Rust strings don't support indexing
  // create a slice instead

  let hello = "Здравствуйте";
  let s = &hello[0..4];
  println!("{s}");

  println!();
  println!("Interating over Strings");
  println!();

  let s = "hello string";

  //for x in s.chars(){
  for x in s.bytes(){
    println!("{x}");
  }

}
