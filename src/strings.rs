// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  println!("===========================");
  println!("Hello from the strings.rs file");
  println!("---------------------------");

  let mut hello = String::from("Hello ");

  // Get length
  println!("Length: {}", hello.len());

  // Push a char
  hello.push('W');

  // Push a string
  hello.push_str("orld!");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Is Empty
  println!("Is Empty: {}", hello.is_empty());

  // Contains
  println!("Contains 'World': {}", hello.contains("World"));

  // Replace
  println!("Replace: {}", hello.replace("World", "There"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(s.len(), 2);
  assert_eq!(s.capacity(), 10);

  println!("{}", hello);
}
