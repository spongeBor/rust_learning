use std::{cell::RefCell, rc::Rc};
use List::{Cons, Nil};
fn main() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  println!("a initial rc count = {}", Rc::strong_count(&a));
  println!("a next item = {:?}", a.tail());

  let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));
  println!("a initial rc count = {}", Rc::strong_count(&a));
  println!("b initial rc count = {}", Rc::strong_count(&b));
  println!("a next item = {:?}", b.tail());

  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));

  // 下面注释打开就是死循环
  // println!("a next item = {:?}", a.tail()) 
}

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}