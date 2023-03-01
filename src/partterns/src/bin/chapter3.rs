///
/// 构造器
/// 

fn main() {

}


//new 创建对象
pub struct Second {
  value: u64
}

impl Second {
  pub fn new (v: u64) -> Self {
    Self { value: v }
  }
}

//default构造
//提供default，就不需要一个不带参数的new构造器
impl Default for Second {
  fn default() -> Self {
      Self { value: 0 }
  }
}
