#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red, Blue, Green,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    // specify the closure expression || self.most_stocked()
    // the closure captures an immutable reference to the self Inventory
    // and passes it through
    // functions are not albe to capture their environment in this way.
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;
    let mut num_green = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Green => num_green += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }

    if num_red > num_blue && num_red > num_green {
      ShirtColor::Red
    } else if num_green > num_blue && num_green > num_red {
      ShirtColor::Green
    } else {
      ShirtColor::Blue
    }
  }
}


fn main() {
  let store = Inventory {
    shirts: vec![
      ShirtColor::Red,
      ShirtColor::Green,
      ShirtColor::Blue,
      ShirtColor::Red,
    ]
  };

  let user1_pref = Some(ShirtColor::Green);
  let giveaway1 = store.giveaway(user1_pref);

  println!("User with user_preference {:?} gets {:?}", 
    user1_pref, giveaway1);

  let user2_pref = None;
  let giveaway2 = store.giveaway(user2_pref);

  println!("User with user_preference {:?} gets {:?}", 
    user2_pref, giveaway2);
}
