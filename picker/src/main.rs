//! Picks a random language for you to complete the next day in
//! Inspired by Low Level Learning's videos (https://www.youtube.com/watch?v=t9CKGXov2Z4)

use std::{io::Write, thread::sleep, time::Duration};

use rand::Rng;
use rand_derive2::RandGen;

#[derive(Debug, strum::Display, RandGen)]
pub enum Language {
    Rust,
    Cpp,
    C,
    Javascript,
    Typescript,
    PHP,
    Crystal,
    Haskell,
    Zig,
    Ruby,
    Go,
    OCaml,
    Clojure,
    Nim,
    Python,
    Dart,
}

fn main() {
    println!("Todays language is...");
    sleep(Duration::from_millis(230));
    println!("Drumroll please...");

    let delay_time = {
        let mut rng = rand::thread_rng();
        rng.gen_range(250..750)
    };

    for _ in 0..3 {
        print!(".");
        std::io::stdout().lock().flush().unwrap();
        sleep(Duration::from_millis(delay_time));
    }
    println!();

    let language: Language = rand::random();

    println!("{language}!!!!!");
}
