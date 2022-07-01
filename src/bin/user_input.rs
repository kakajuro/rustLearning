use std::io;

fn main() {
  println!("Program runninng...");

  let mut string_inputted:String = String::new();
  io::stdin().read_line(&mut string_inputted).expect("Failed to read the line");
  
  println!("You typed: {}", string_inputted);
}