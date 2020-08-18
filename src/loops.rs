pub fn run() {
  // let mut sum = 0;
  // let mut counter = 0;
  // loop {
  //   if counter <= 100 {
  //     sum += counter;
  //     counter += 1;
  //   } else {
  //     break;
  //   }
  // }
  // println!("{}", sum);

  // let mut count = 1;
  // while count <= 100 {
  //   if count % 15 == 0 {
  //     println!("fizzbuzz");
  //   } else if count % 3 == 0 {
  //     println!("fizz");
  //   } else if count % 5 == 0 {
  //     println!("buzz");
  //   } else {
  //     println!("{}", count);
  //   }
  //   count += 1;
  // }

  let n = 100;
  for count in 0..n + 1 {
    if count % 15 == 0 {
      println!("fizzbuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", count);
    }
  }
}
