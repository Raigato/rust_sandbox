// Functions - Used to store blocks of code for re-use

pub fn run() {
  println!("===========================");
  println!("Hello from the functions.rs file");
  println!("---------------------------");

  greeting("Hello", "Jane");

  // Bind function values to variables
  let get_sum = add(5, 5);
  assert_eq!(get_sum, 10);

  // Closure
  let c: i32 = 10;
  let add_nums = |a: i32, b: i32| a + b + c;
  assert_eq!(add_nums(3, 3), 16);
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name)
}

fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
