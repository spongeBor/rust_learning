use std::ops::Deref;

fn main() { 
  // let x = 5;
  // // let y = &x;
  // let y = Box::new(x);
  // assert_eq!(5, x);
  // assert_eq!(5, *y);
  // println!("{x}, {y}");

  let x= 5;
  let y = MyBox::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y); // *y 会被隐式的转换为*(y.deref())

  let m = MyBox::new(String::from("Rust"));
  // 进行了自动解引用
  // 否则需要：hello(&(*m)[..])
  hello(&m)
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
impl<T> Deref for MyBox<T> {
  type Target = T; // 关联类型
  fn deref(&self) -> &T {
    &self.0
  }
}
fn hello(name: &str){
  println!("Hello, {}!", name);
}