#[derive(Debug)]
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

// the tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("some_username123"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("another_person@example.com");
  println!("{:?}", user1);

  // creating instances from other instances
  // struct update syntax
  let user2 = User{
    email: String::from("this_is_nice@example.com"),
    ..user1
  };

  println!("{:?}", user2);

  //println!("{:?}", user1);

  let someColor = Color(4,4,4);

  println!("{:?}", someColor);
}

fn build_user(email: String, username: String) -> User {
  // using the field init shorthand
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}