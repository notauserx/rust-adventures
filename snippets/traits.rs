// cargo-deps: chrono = "0.4"
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// hashbang.
extern crate chrono;

use chrono::Datelike;

struct Album {
    title: String,
    artist: String,
    release_year: u32,
    genre: String,
}

// a trail is like a type class
trait Details {
    fn description(&self) -> String;
    fn years_since_release(&self) -> u32;
}

impl Details for Album {
    fn description(&self) -> String {
        return format!(
            "{}, released in {}, genre {} by {}.",
            self.title, self.release_year, self.genre, self.artist
        );
    }

    fn years_since_release(&self) -> u32 {
        return chrono::Utc::now().year() as u32 - self.release_year;
    }
}

fn main() {
    let album = Album {
        title: "Fear of the dark".to_string(),
        artist: "Iron Maiden".to_string(),
        release_year: 1992,
        genre: "Heavy Metal".to_string(),
    };
    
    println!("{}", chrono::Utc::now().year());
    println!("\n{}", album.description());
    println!("\n{} years since release.", album.years_since_release());
}
