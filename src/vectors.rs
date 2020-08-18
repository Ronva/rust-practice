use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!("{:?}", numbers);
  println!("{}", numbers[3]);

  numbers[2] = 20;

  numbers.push(6);

  println!("{:?}", numbers);
  println!("{}", numbers[2]);

  println!("{} bytes", mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[1..3];
  println!("{:?}", slice);

  for x in numbers.iter() {
    println!("{}", x + 4);
  }
}
