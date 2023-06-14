fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let mut largest = number_list[0];

  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest number is {}", largest);
  let number_list = vec![34, 50, 25, 100, 65];
  let result = cul_largest(&number_list);
  println!("The largest number is {}", result);
  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = cul_largest(&number_list);
  println!("The largest number is {}", result);

  println!("The largest number is {}", largest);
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest_generic(&number_list);
  println!("The largest number is {}", result);
  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest_generic(&number_list);
  println!("The largest number is {}", result);
}
fn cul_largest(list: &[i32]) ->i32 {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// 泛型
fn largest_generic<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}