#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)] // so we can inspect the state in a minute
enum Coin {
  Penny, Nickel, Dime, Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    //this is called a match arm
    Coin::Penny => 1,
    // arm has two parts, a pattern and some code.
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

fn main() {
  let alabamaQuarter = Coin::Quarter(UsState::Alabama);

  println!("{}", value_in_cents(alabamaQuarter));
}