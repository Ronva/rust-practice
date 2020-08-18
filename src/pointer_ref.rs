pub fn run() {
  // Primitive Array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("{:?}", (arr1, arr2));
  // Non-primite
  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1; // & for reference

  println!("{:?}", (&vec1, vec2));
}
