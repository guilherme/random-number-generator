extern crate rand;
use std::io;
use rand::{thread_rng, Rng};
fn main() {
  let stdin = io::stdin();
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {},
    Err(error) => println!("error: {}", error),
  }
  let numbers = input.trim().parse::<i32>().unwrap();
  let mut i = 0;
  let mut rng = thread_rng();
  while i <= numbers {
    let n : u32 = rng.gen();
    println!("{}", n);
    i += 1;
  }
}
