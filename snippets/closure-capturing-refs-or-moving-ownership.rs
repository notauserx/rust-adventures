/*
*/
fn main() {
  //An example of a closure that captures an immutable reference
  println!("Capturing an immutable reference");
  let list = vec![1,2,3];
  println!("Before defining closures {:?}", list);

  let only_borrows = || println!("From closure {:?}", list);

  println!("Before calling closure {:?}", list);
  only_borrows();
  println!("After calling closure {:?}", list);

  // capturing a mutable reference
  println!("Capturing a mutable reference");
  let mut list1 = vec![1,2,3];

  println!("Before defining closures {:?}", list1);

  let mut borrows_mutably = || list1.push(4);

  // uncommenting the following line gives a compile error
  //println!("Before calling borrows_mutably {:?}", list1);
  borrows_mutably();
  println!("After calling borrows_mutably {:?}", list1);


  // an example of closure taking ownership

  println!();
  println!("Closure taking ownership");
  let list2 = vec![1, 2, 3];

  println!("Before calling closure {:?}", list2);

  std::thread::spawn(move || println!("from thread {:?}", list2))
    .join()
    .unwrap();
}