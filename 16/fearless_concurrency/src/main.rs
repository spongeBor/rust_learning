use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}, vec, rc::Rc};

fn main() {
  //1
  // let handle = thread::spawn(|| {
  //   for i in 1..10 {
  //     println!("hi number {} from the spawned thread!", i);
  //     thread::sleep(Duration::from_millis(1));
  //   }
  // });
  // // handle.join().unwrap();

  // for i in 1..5 {
  //   println!("h1 number {} from the main thread!", i);
  //   thread::sleep(Duration::from_millis(1));
  // }
  // handle.join().unwrap();

  //2
  // let v = vec![1,2,3];
  // let handle = thread::spawn( move || {
  //   println!("Here's a vector: {:?}", v);
  // });
  // handle.join().unwrap();

  //3 消息传递
  // let (tx, rx) = mpsc::channel();
  // thread::spawn(move || {
  //   let val = String::from("hi");
  //   tx.send(val).unwrap();
  //   // println!("val is {}", val);
  // });
  // let received = rx.recv().unwrap();
  // println!("Got: {}", received);

  //4 多个消息发送
  // let (tx, rx) = mpsc::channel();
  // let tx1 = mpsc::Sender::clone(&tx);
  // thread::spawn(move || {
  //   let vals = vec![
  //     String::from("hi"),
  //     String::from("from"),
  //     String::from("the"),
  //     String::from("thread"),
  //   ];
  //   for val in vals {
  //     tx1.send(val).unwrap();
  //     thread::sleep(Duration::from_secs(1));
  //   }
  // });
  // thread::spawn(move || {
  //   let vals = vec![
  //     String::from("more"),
  //     String::from("messages"),
  //     String::from("for"),
  //     String::from("you"),
  //   ];
  //   for val in vals {
  //     tx.send(val).unwrap();
  //     thread::sleep(Duration::from_secs(1));
  //   }
  // });
  // for received in rx {
  //   println!("Got: {}", received);
  // }

  //5共享状态
  // let m = Mutex::new(5);
  //  {
  //    let mut num = m.lock().unwrap();
  //    *num = 6;
  //  }
  //  println!("m = {:?}", m);

  let counter = Arc::new(Mutex::new(0)); // Arc 是在并发中使用的Rc
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });
    handles.push(handle);
  }
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}
