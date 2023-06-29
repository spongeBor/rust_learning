use std::io;

fn main() {
  type Kilometers = i32; // Kilometers 和 i32是同义词
  let x:i32 = 5;
  let y: Kilometers = 5; 
  println!("x + y = {}", x + y);

  type Thunk = Box<dyn Fn() + Send + 'static>;
  let f:Thunk = Box::new(|| println!("hi"));

  fn takes_long_type(f: Thunk) {
    //dfs
  }
  fn returns_long_type() -> Thunk {
    //dfs
    Box::new(|| ())
  }
}