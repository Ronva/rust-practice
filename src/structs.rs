struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

struct TupleColor(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct a person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }
  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string()
  }
  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };
  println!("R{} G{} B{}", c.red, c.green, c.blue);
  c.red = 200;
  println!("R{} G{} B{}", c.red, c.green, c.blue);

  let mut t = TupleColor(255, 0, 0);
  println!("R{} G{} B{}", t.0, t.1, t.2);
  t.1 = 100;
  println!("R{} G{} B{}", t.0, t.1, t.2);

  let mut p = Person::new("John", "Doe");
  println!("{}", p.full_name());
  p.set_last_name("Smith");
  println!("{}", p.full_name());
  println!("{:?}", p.to_tuple());
}
