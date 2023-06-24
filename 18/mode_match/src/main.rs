fn main() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }


  let mut stack = Vec::new();
  stack.push(1);
  stack.push(2);
  stack.push(3);
  while let Some(top) = stack.pop() {
    println!("{}", top);
  }


  let v = vec!['a', 'b', 'c'];
  for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
  }


  let x = 5;
  let (x , y, z) = (1, 2, 3);
  // let (x, y) = (1, 2, 3); // 数量不同会报错
  let (x, y, _) = (1, 2, 3);
  let (x, ..) = (1, 2, 3);


  let point = (3, 5);
  print_coordinates(&point);


  // 匹配字面量
  let x= 1;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything")
  }


  // 匹配命名变量
  let x = Some(5);
  let y = 10;
  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }
  println!("at the end: x = {:?}, y = {:?}", x, y);


  // 多重模式
  let x = 1;
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }


  // 使用...来匹配区间
  let x = 5;
  match x {
    1 ..= 5 => println!("one through five"), // 1..=5 相当于 1 | 2 | 3 | 4 | 5
    _ => println!("something else"),
  }
  let x = 'c';
  match x {
    'a' ..= 'j' => println!("early ASCII letter"),
    'k' ..= 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }


  // 解构
  // 解构结构体
  let p = Point {x: 0, y: 7};
  let Point {x: a, y: b} = p;
  assert_eq!(0, a);
  assert_eq!(7, b);
  let Point {x, y} = p;
  assert_eq!(0, x);
  assert_eq!(7, y);
  match p {
    Point {x, y: 0} => println!("On the x axis at {}", x),
    Point {x: 0, y} => println!("On the y axis at {}", y),
    Point {x, y} => println!("On neither axis at ({}, {})", x, y),
  }
  // 解构枚举
  let msg = Message::ChangeColor(0, 160, 255);
  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    },
    Message::Move { x, y } => {
      println!(
        "Move in the x direction {} and in the y direction {}",x, y
      )
    },
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!(
        "Change the color to red {}, green {}, adn blue {}",
        r,
        g,
        b
      )
    }
  }


  // 可以使用匹配守卫添加条件 1
  let num = Some(4);
  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => ()
  }
  // 2
  let x = Some(5);
  let y = 10;
  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {:?}", n),
    _ => println!("Default case, x = {:?}", x)
  }
  println!("at the end: x = {:?}, y = {:?}", x, y);


  // @绑定
  let msg = Message2::Hello { id: 5 };
  match msg {
    Message2::Hello {id: id_variable @ 3..=7} => {
      println!("Found an id in range: {}", id_variable)
    },
    Message2::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    },
    Message2::Hello { id } => {
      println!("Found some other id: {}", id)
    }
  }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Current location: ({}, {})", x, y);
}
struct Point {
  x: i32,
  y: i32,
}

enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}

enum Message2 {
  Hello {id: i32},
}

