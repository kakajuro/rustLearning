use std::io;
use std::f32;

struct CircleData {
  area: f32,
  radius: f32,
  diameter: f32
}

fn given_area(a:f32) -> CircleData {

  let r:f32 = (a / std::f32::consts::PI).sqrt();
  let d:f32 = r*2.0;

  return CircleData { area: a, radius: r, diameter: d }

}

fn given_radius(r:f32) -> CircleData {

  let d:f32 = r*2.0;
  let a:f32 = r.powi(2) * std::f32::consts::PI;

  return CircleData { area: a, radius: r, diameter: d }

}

fn given_diameter(d:f32) -> CircleData {

  let r:f32 = d/2.0;
  let a:f32 = r.powi(2) * std::f32::consts::PI;

  return CircleData { area: a, radius: r, diameter: d }

}

fn main() {
  
  println!("Enter r, d or a (area, diameter or radius):");

  let mut choice_input:String = String::new();
  io::stdin().read_line(&mut choice_input).expect("Failed to read input");
  let choice:String = choice_input.trim().parse().unwrap();

  if choice.as_str() != "a" || choice.as_str() != "d" || choice.as_str() != "d" {
    println!("Choice not available");
    std::process::exit(1);
  }

  println!("Please enter a value for that:");

  let mut value_input:String = String::new();
  io::stdin().read_line(&mut value_input).expect("Failed to read input");
  let value:f32 = value_input.trim().parse().unwrap();

  let data = match choice.as_str() {
      "r"=> given_radius(value),
      "d"=> given_diameter(value),
      "a"=> given_area(value),
      &_=>panic!("Oh no something bad has happened!")
  };
 
  println!("The area is: {}", data.area);
  println!("The radius is: {}", data.radius);
  println!("The diameter is: {}", data.diameter);

}