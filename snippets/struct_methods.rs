#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}


// Each struct is allowed multiple impl blocks
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 3,
    height: 4,
  };

  let rect2 = Rectangle {
    width: 2,
    height:3,
  };

  let sq1 = Rectangle::square(5);

  println!("The area of rect is {} sq pixels", rect1.area());

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

  println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

  println!("Can sq1 hold rect1? {}", sq1.can_hold(&rect1));

}