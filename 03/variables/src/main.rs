fn main() {
  // 数值中插入下划线，提高可读性
  const MAX_POINTS: u32 = 100_000;
  let mut x = 5;
  println!("The value of is : {}", x);
  x = 6;
  println!("The value of is : {}", x);

  // 变量隐藏
  // 相当于创建新的变量，但名字被重用
  let y = 5;
  let y = y + 1;
  let y = y * 2;
  println!("The value of x is: {}", y);

  // 可以同时改变类型
  let spaces = "   ";
  let spaces = spaces.len();
  // mut不能改变类型
  // let mut spaces2 = "   ";
  // spaces2 = spaces2.len();

  // 标量类型
  // 整数，浮点数、布尔值、字符
  let guess: u32 = "42".parse().expect("Not a number!");
  let diff = 95.5 / 4.3;
  println!("{diff}");

  // 复合类型
  // 元组
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {}", y);
  let x = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
  println!("{}, {}, {}",five_hundred, six_point_four, one);
  //数组
  let a = [1,2,3,4,5];
  let a = [3; 5];
  let first = a[0];
  let second = a[1];
  another_function(5);
}

fn another_function(x: i32) {
  println!("Another function.{x}");
}