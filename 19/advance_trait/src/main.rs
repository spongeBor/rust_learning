use core::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}
impl Add for Point {
  type Output = Point;
  // rhs == right-handle side
  fn add(self, rhs: Self) -> Self::Output {
      Point {
        x: self.x + rhs.x,
        y: self.y + rhs.y,
      }
  }
}
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + self.0 * 1000)
    }
}


trait Pilot {
  fn fly(&self);
}
trait Wizard {
  fn fly(&self);
}
struct Human;
impl Pilot for Human {
  fn fly(&self) {
      println!("This is your captain speaking.")
  }
}
impl Wizard for Human {
  fn fly(&self) {
      println!("Up!")
  }
}
impl Human {
    fn fly(&self) {
      println!("*waving arms furiously");
    }
}


trait Animal {
  fn baby_name() -> String;
}
struct Dog;
impl Dog {
  fn baby_name() -> String{
    String::from("Spot")
  }
}
impl Animal for Dog {
  fn baby_name() -> String {
      String::from("puppy")
  }
}


// 孤儿模式会阻止我们在 使用外部trait对外部类型进行实现
// 使用newtype模式在外部类型上实现外部trait
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
  }
}


fn main() {
  assert_eq!(Point {x: 1, y: 0} + Point {x: 2, y: 3}, Point {x: 3, y: 3});
  println!("{:?}", Millimeters(20) + Meters(2));
  // println!("{:?}", Meters(2) + Millimeters(20)); // 会报错

  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();

  println!("A baby dog is called a {}", Dog::baby_name());
  // println!("A baby dog is called a {}", Animal::baby_name()); // 报错
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // 完全限定语法

  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
