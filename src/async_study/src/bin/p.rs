use std::{ptr, ops::Deref};
fn main() {
  let data = move_creates_issue();
  println!("data:{:?}",data);
  data.print_name();
}

#[derive(Debug)]
struct SelfReference {
    name: String,
    name_ptr: *const String,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_ptr: ptr::null(),
        }
    }
    pub fn init(&mut self) {
        self.name_ptr = &self.name as *const String
    }

    pub fn print_name(&self) {
        println!(
            "struct{:p}: (name:{:p},name_ptr:{:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            &self.name_ptr,
            self.name,
            unsafe { &*self.name_ptr }
        )
    }
}

fn move_creates_issue() -> SelfReference {
  let mut data = SelfReference::new("abc");
  data.init();
  data.print_name();
  let data = move_it(data);
  data.print_name();
  data
}

fn move_it(data:SelfReference) -> SelfReference {
  data
}
