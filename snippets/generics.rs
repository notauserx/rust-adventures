fn largest<T>(lst: &[T]) -> &T 
where T: std::cmp::PartialOrd {
  let mut largest = &lst[0];

  for item in lst {
    if item > largest {
      largest = item;
    }
  }
    largest
}

struct Point<T> {
  x: T,
  y: T,
}

impl <T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

struct ComplexPoint<T, U> {
  x: T,
  y: U,
}
fn main() {
  let number_list = vec![1,2,3,4,5,6,2,3,4,1];

  /*
  let mut largest = &number_list[0];

  for number in &number_list {
    if number > largest {
      largest = number;
    }
  }

  println!("The largest number is: {largest}");
  */
  let largest_number = largest(&number_list);

  println!("the largest number is {largest_number}");

  let char_list = vec!('a', 'x', 'b', 'z');
  let largest_char = largest(&char_list);

  println!("the largest char is {largest_char}");

  let a_point = Point { x: 3.1, y: 4.2 };
}