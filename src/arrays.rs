// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
  println!("===========================");
  println!("Hello from the arrays.rs file");
  println!("---------------------------");

  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Re-assign a value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  let numbers2: [i32; 4] = [1, 2, 3, 4];
  println!("Array occupies {} bytes", mem::size_of_val(&numbers2));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}
