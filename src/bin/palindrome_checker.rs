use std::io;

fn palindrome_finder (word_length:usize, input:String) -> bool {

  let mut result:bool = false;
  
  let split_string = input.split_at(word_length/2);
  
  if word_length % 2 > 0 {

    let split_string_2 = input.split_at(word_length/2 + 1);

    let second_el:String = split_string.1.chars().rev().collect();
    let first_el:String = split_string_2.0.to_string();

    if first_el == second_el {
      result = true;
    } else {
      result = false
    }
    
  } else {
    
    let second_el:String = split_string.1.chars().rev().collect();
    let first_el:String = split_string.0.to_string();

    if first_el == second_el {
      result = true;
    } else {
      result = false
    }

  }

  return result;

}

fn main() {
  
  let mut input:String = String::new();
  
  println!("Enter a word to be palindrome tested:");

  io::stdin().read_line(&mut input).expect("Input could not be read");
  let word:String = input.trim().parse().unwrap();

  let word_length = word.chars().count();

  let result:bool = palindrome_finder(word_length, word);


  if result {
    println!("Palindrome found!");
  } else {
    println!("Palindrome not found!");
  }

}