// Reference Pointers - Point to a ressource in memory

pub fn run() {
  println!("===========================");
  println!("Hello from the pointer_refs.rs file");
  println!("---------------------------");

  // Primitive Array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2));

  // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a refrence (&) to point to the resource
  // Vector
  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, vec2));
}
