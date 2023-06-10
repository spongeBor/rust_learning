use std::net::{Ipv4Addr, Ipv6Addr};

enum IpAddrKind {
  V4,
  V6
}
fn route(ip_type: IpAddrKind) {}
fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
  route(IpAddrKind::V4);
  route(IpAddrKind::V6);

  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1")
  };
  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1")
  };


  enum IpAddr2 {
      V4(String),
      V6(String)
  }
  let home = IpAddr2::V4(String::from("127.0.0.1"));
  let lookback = IpAddr2::V6(String::from("::1"));


  enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
  }
  let home = IpAddr3::V4(127, 0, 0, 1);
  let lookback = IpAddr3::V6(String::from("::1"));


  // 内置
  enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr)

  }

  
  // OPTION<T>
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number:Option<i32> = None;


  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn Value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
          println!("Lucky penny!");
          1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
  }

  fn plus_one(x: Option<i32>) ->Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("{:?}, {:?}", six, none);


  let some_u8_value = 0u8;
  match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => ()
  }

  let some_u8_value = Some(0u8);
  if let Some(3) = some_u8_value {
    println!("three")
  }

  // let mut count = 0;
  // if let Coin::Quarter(state) = coin {
  //   println!("State quarter from {:?}", state);
  // } else {
  //   count += 1;
  // }


}
