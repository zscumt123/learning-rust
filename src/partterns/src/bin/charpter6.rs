///
/// 分解结构体


fn main() {
  
}


struct A {
  f1: u32,
  f2: u32,
  f3: u32
}

fn foo(a: &mut A) -> &u32 {
  &a.f2
}
fn bar(a: &mut A) -> u32 {
  a.f1 + a.f3
}
fn baz(a: &mut A) {
  let x = foo(a);
  //多个可变借用
  // let y = bar(a);
  println!("x:{}",x);
}

///
/// 分解为多个结构体
/// 

struct A1 {
  b: B,
  c: C,
}
struct B {
  f2: u32,
}
struct C {
  f1: u32,
  f3: u32,
}

// These functions take a B or C, rather than A.
fn foo1(b: &mut B) -> &u32 { &b.f2 }
fn bar1(c: &mut C) -> u32 { c.f1 + c.f3 }

fn baz1(a: &mut A1) {
  let x = foo1(&mut a.b);
  // Now it's OK!
  let y = bar1(&mut a.c);
  println!("{}", x);
}
