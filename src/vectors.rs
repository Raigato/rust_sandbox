// Vectors - Resizable arrays

use std::mem;

pub fn run() {
  println!("===========================");
  println!("Hello from the vectors.rs file");
  println!("---------------------------");

  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Re-assign a value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  println!("{:?}", numbers);

  // Pop off last value
  numbers.pop();
  println!("{:?}", numbers);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", numbers);
}
