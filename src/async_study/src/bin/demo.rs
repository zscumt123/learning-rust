use std::ops::Deref;

fn main() {
  let s = Box::new(String::from("value"));
let n = s.to_string();
}

#[derive(Debug)]
struct Person {
  name: String,
  age: u8
}
impl Person {
  fn display(self: &mut Person,age:u8) {
    let Person {name, age} = self;
  }
}
