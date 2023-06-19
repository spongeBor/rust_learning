fn main() {
  let v1 = vec![1,2,3];
  let iter = v1.iter();

  for val in iter {
    println!("Got: {}", val);
  }
}
