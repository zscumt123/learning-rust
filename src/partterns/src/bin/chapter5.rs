///
/// option 迭代器
/// 
fn main () {
  let t = Some("t");
  let mut chars = vec!["1","2","3"];
   chars.extend(t);
  for i in chars {
    println!("{}",i)
  }
}
