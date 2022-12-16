use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..=10);

  loop {
    println!("Guess a number");
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    //.expect("Please type a number.");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("too small"),
      Ordering::Greater => println!("too big"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
