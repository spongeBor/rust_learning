fn main() {
    // let s = "hello";
    // let s = String::from("hello");
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{s}")

    // let s1 = String::from("hello!");
    // let s2 = s1;
    // println!("{s2}");

    // let s1 = String::from("hello!");
    // let s2 = s1.clone();
    // println!("{s1}, {s2}");

    // let s = String::from("hello");
    // takes_ownership(s);
    // let x = 5;
    // makes_copy(x);
    // fn takes_ownership(some_string: String) {
    //   println!("{some_string}");
    // }
    // fn makes_copy(some_integer: i32) {
    //   println!("{some_integer}");
    // }

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // fn gives_ownership() -> String{
    //   let some_string = String::from("hello");
    //   some_string
    // }
    // fn takes_and_gives_back(a_string: String)->String {
    //   a_string
    // }
    // println!("{s3}, {s1}")

    // 引用与借用
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{s1}' is {len}.");
    // fn calculate_length(s: &String) -> usize {
    //     s.len()
    // }
    // 可变引用
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}")

    // println!("{s}");
    // change(&mut s);
    // fn change(some_string: &mut String) {
    //   some_string.push_str(", world");
    // }
    // println!("{s}");

    // 切片
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{hello},{world}");

    fn first_world(s: &String) -> &str {
      let bytes = s.as_bytes();
      for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[0..i];
        }
      }
      &s[..]
    }
    let mut s = String::from("hello world");
    let word = first_world(&s);
    // s.clear();
    println!("the first word is : {word}");

}
