use std::io;

fn main() {
  println!("Enter your name");
  let mut my_name = String::new();
  io::stdin()
    .read_line(&mut my_name)
    .expect("Failed to read line");
  println!("Your name is: {}", my_name);

  let mut my_age = String::new();
  io::stdin()
    .read_line(&mut my_age)
    .expect("Failed to read line");
  println!("Your age is: {}", my_age);

  let mut age: u16 = my_age.trim().parse().expect("Parsed incorrectly");
  age += 1;
  println!("Your updated age: {}", age);
}
