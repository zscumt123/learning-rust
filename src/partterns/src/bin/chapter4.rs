///
/// 
/// 用mem::{take(_), replace(_)}在修改枚举变体时保持值的所有权
/// 
/// 
fn main () {

}

use std::mem;
enum  MyEnum {
    A{name: String, x: u8},
    B{name: String}
}
fn a_to_b(e: &mut MyEnum ) {
  *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
    // MyEnum::B { name: mem::take(name) }
    MyEnum::B { name: mem::replace(name, String::default()) }
  } else {
    return 
  }
}
