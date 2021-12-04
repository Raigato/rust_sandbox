// Structs - Used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct Person
  fn new(first_name: &str, last_name: &str) -> Person {
    return Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
    };
  }

  fn full_name(&self) -> String {
    return format!("{} {}", self.first_name, self.last_name);
  }

  // Set last name
  fn set_last_name(&mut self, last_name: &str) {
    self.last_name = last_name.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    return (self.first_name, self.last_name);
  }
}

pub fn run() {
  println!("===========================");
  println!("Hello from the structs.rs file");
  println!("---------------------------");

  // Color
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  c.red = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  // Color2
  let mut c2 = Color2(255, 0, 0);

  c2.0 = 202;

  println!("Color: {} {} {}", c2.0, c2.1, c2.2);

  // Person
  let mut p = Person::new("John", "Doe");

  println!("Person {} {}", p.first_name, p.last_name);

  p.set_last_name("Williams");
  println!("Full name: {}", p.full_name());

  println!("To Tuple: {:?}", p.to_tuple());
}
