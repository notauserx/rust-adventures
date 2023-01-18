use rand::Rng;
use std::{cmp::Ordering, io};

const LOWER_LIMIT: i32 = 1;
const UPPER_LIMIT: i32 = 10;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < LOWER_LIMIT || value > UPPER_LIMIT {
      panic!("Guess value must be between {LOWER_LIMIT} and {UPPER_LIMIT}, got {value}.");
    }

    Guess { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

fn main() {
  let secret_number = rand::thread_rng().gen_range(LOWER_LIMIT..=UPPER_LIMIT);

  let secret_guess = Guess::new(secret_number);

  loop {
    println!("Guess a number");
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    println!("You guessed: {}", guess);

    let guess: Guess = match guess.trim().parse::<i32>() {
      Ok(num) => Guess::new(num),
      Err(_) => continue,
    };

    match guess.cmp(&secret_guess) {
      Ordering::Less => println!("too small"),
      Ordering::Greater => println!("too big"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
