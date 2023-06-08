#[derive(Debug)] // 需要手动加入，才能打印
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  // (方法) 总是被定义在结构体中
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle)->bool {
    self.width > other.width && self.height > other.height
  }
}
// 写的多个impl会被合并
impl Rectangle {
  // 关联函数，因为不会作用于某个具体的结构体实例
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}
fn main() {
  let width1 = 30;
  let height1 = 30;
  let rect1 = (30, 50);
  let rect2 = Rectangle {width: 30, height: 30};
  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );
  println!(
    "The area2 of the rectangle is {} square pixels.",
    area2(rect1)
  );
  println!(
    "The area3 of the rectangle is {} square pixels.",
    area3(&rect2)
  );
  println!(
    "rect3 is Rectangle {:?}", rect2
  );
  println!(
    "rect3 is Rectangle {:#?}", rect2
  );
  println!("{}", rect2.area());

  let rect3 = Rectangle {width: 30, height: 50};
  let rect4 = Rectangle {width: 10, height: 40};
  let rect5 = Rectangle {width: 60, height: 45};
  println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
  println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

  let rect6 = Rectangle::square(40);
  println!("rect6 is {:#?}", rect6);

}
fn area(width: u32, height: u32) -> u32 {
  width * height
}
fn area2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}