use std::io;
use rand::Rng;

fn main() {

  let mut rng = rand::thread_rng();

  println!("Enter the maximum integer...");

  let mut user_input = String::new();
  io::stdin().read_line(&mut user_input).expect("Failed to read line");

  let int_input:i32 = user_input.trim().parse().unwrap();

  let random_int = rng.gen_range(0..int_input);
  
  // Ln 16 not working
  println!("Random integer: {}", random_int);

}