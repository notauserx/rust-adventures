use std::collections::HashMap;

fn main() {

  // creating a new hash map
  let mut numbers = HashMap::new();

  let threeValue = String::from("three");

  numbers.insert(1, String::from("one"));
  numbers.insert(2, String::from("two"));
  numbers.insert(3, threeValue);

  // hashmaps allocates memory on the heap, just like vectors

  // for owned values like string, the values will be moved
  // and the hash map will be the owner of those values
  // making this line not compile
  //println!("{threeValue}");

  let one = numbers.get(&1).unwrap();
  println!("{one}");

  let undefined = String::from("undefined");
  let missing = numbers.get(&100).unwrap_or(&undefined);
  println!("{missing}");

  // iterate over the key value pair
  for (key, value) in &numbers {
    println!("{key}: {value}");
  }

  update_value_based_on_old_value();
}

fn update_value_based_on_old_value() {
  let text = "some text and some other stuff";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);
}