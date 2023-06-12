use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
fn main() {
  // panic!("crash and burn");
  let a = overview();
  println!("{a}");
  // let a = get_file();
  // println!("{:?}", a);
  let b = Guess::new(12323);
  println!("{:?}", b);
  let a = get_file_02();
  // let a = get_file_03();
  println!("{:?}", a);
  
}
fn overview() -> i32 {
  let v = vec![1, 2, 3];

  // v[99]
  v[1]
}

fn get_file() ->File {
  let f = File::open("hello.txt");
  let f = match f {
      Ok(file) => file,
      Err(error) => {
        panic!("There was a problem opening the file: {:?}", error);
      }
  };
  f
}

fn get_file_02() -> File {
  let f = File::open("hello.txt");
  let f = match f {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        other_error => panic!("There was a problem opening the file: {:?}", other_error),
      }
  };
  f
}

fn get_file_03() {
  let f = File::open("hello.txt").map_err(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Tried to create file but there was a problem: {:?}", error)
      })
    } else {
      panic!("There was a problem opening the file: {:?}", error);
    }
  });

}

#[derive(Debug)]
pub struct Guess {
  value: i32,
}
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}", value);
    }
    Guess {
      value
    }
   }
  pub fn value(&self) -> i32 {
    self.value
  }
}