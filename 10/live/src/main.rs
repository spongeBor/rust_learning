use std::fmt::Display;

fn main() {
    // 1
    // let r;
    // {
    //   let x = 5;
    //   r = &x;
    // }
    // println!("r: {}", r);

    //2 函数中泛型的生命周期
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The logest string is {}", result);
    }

    //4结构体定义中的生命周期标注 
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};

    // 6表态生命周期
    let s: &'static str = "I have a static lifetime.";
}
//2 函数中泛型的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 3
fn long<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// fn long22<'a>(x: &str, y: &str) -> &'a str {}

// 4 结构体定义中的生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//5 方法定义中的生命周期
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {3}
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

//7 同时使用泛型、trait约束、生命周期
fn longest_witch_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
  where T: Display {
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}