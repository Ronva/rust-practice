pub fn run() {
  let sum: u64 = sum_to_n(10);
  println!("{}", sum);

  let add_nums = |n1: i32, n2: i32| n1 + n2;
  println!("{}", add_nums(15, 234));
}

fn sum_to_n(n: u64) -> u64 {
  let mut sum: u64 = 0;
  let mut counter: u64 = 1;

  loop {
    if counter <= n {
      sum += counter;
      counter += 1;
    } else {
      break;
    }
  }
  sum
}
