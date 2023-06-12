use std::{fmt::format, collections::HashMap};

fn main() {
  let v: Vec<i32> = Vec::new(); // 动态数组
  let v = vec![1,2,3]; // 简写方法
  let mut v = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7); 
  v.push(8);
  println!("{:?}", v);
  v.pop();
  println!("{:?}", v);

  let third: &i32 = &v[2];
  println!("the third element is {}", third);
  match v.get(2) {
    Some(third) => println!("the third element is {}", third),
    None => println!("There is no third element.")
  }

  // 不存在的元素引用
  // let does_not_exist = &v[100]; // 会报错
  let does_not_exist = v.get(100);
  println!("{:?}", does_not_exist);
  // test01()
  for_each();
  enum_test();
  string_text();
  map();
}

fn test01() {
  // 不能在同一个作用域中同时拥有可变引用和不可变引用
  let mut v = vec![1,2,3,4,5];
  let first = &v[0];
  // v.push(6); 
  println!("{:?}", first)
}
fn for_each() {
  let v = vec![100, 32, 57];
  // for i in v {
    for i in &v {
    println!("{}", i)
  }
  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
  }
}
fn enum_test() {
  #[derive(Debug)]
  enum SpreadsheetCeil {
    Int(i32),
    Float(f64),
    Text(String)
  }
  let row = vec![
    SpreadsheetCeil::Int(3),
    SpreadsheetCeil::Text(String::from("blue")),
    SpreadsheetCeil::Float(10.12)
  ];
  println!("{:?}", row)
}

fn string_text() {
  let mut s = String::new();
  let data = "initial contents";
  let data = "initial contents".to_string();
  
  let mut s = String::from("foo");
  s.push_str("bar");
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("{s2}");

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;
  // println!("{s1}")
  println!("{s3}");

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // let s = s1 + &s2 + &s3;
  let s = format!("{}-{}-{}", s1, s2, s3);
}

fn map() {
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");
  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  println!("{:?}", map);
}