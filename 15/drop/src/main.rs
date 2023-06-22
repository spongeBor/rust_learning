fn main() {
  let c = CustomSmartPointer {data: String::from("my stuff")};
  let d = CustomSmartPointer {data: String::from("other stuff")};
  drop(c); // 可以用来提前释放
  println!("CustomsmartPointers created.");
}
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer whth data `{}`!", self.data)
  }
}
