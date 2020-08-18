use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);
  println!("{}", numbers[3]);

  numbers[2] = 20;

  println!("{:?}", numbers);
  println!("{}", numbers[2]);

  println!("{} bytes", mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[1..3];
  println!("{:?}", slice);
}
