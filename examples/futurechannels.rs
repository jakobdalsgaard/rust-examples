extern crate futures;

use std::{time, thread};
use futures::*;
use futures::sync::oneshot;
use futures::sync::mpsc;
use futures::sync::mpsc::Sender;

fn main () {

  println!("Example 1");
  let (sender, receiver) = oneshot::channel();
  let child = thread::spawn(|| {
    receiver.map(|i| {
      println!("got: {}", i);
    }).wait();
  });

  let () = sender.complete(3);
  let _ = child.join();


  println!("Example 2");

  // futured channel is not necessarily thread secured
  let (sender2, mut receiver2) = mpsc::channel(0);

  let child2 = thread::spawn(move || {
    println!("sending 1");
    sender2.send(1).and_then(|tx| { println!("sending 2"); tx.send(2) } ).wait();
  });

  thread::sleep(time::Duration::from_millis(100));

  while match receiver2.poll() {
    Ok(Async::Ready(None)) => { println!("All done..."); false },
    Ok(Async::Ready(Some(x))) => { println!("got: {}", x); true },
    Ok(Async::NotReady) => { println!("not read"); true},
    Err(s) => { println!("some error {:?}", s); true },
  } {
    println!("Polling...");
    thread::sleep(time::Duration::from_millis(10));
  }


  println!("Example 3");
  let (sender3, receiver3) = mpsc::channel(4);

  sender3.send(5).and_then(|tx| tx.send(6)).wait();

  let (item, rest) = receiver3.into_future().wait().ok().unwrap();

  match item {
   None => println!("Got none"),
   Some(x) => println!("Got some value: {}", x),
  }

  let (item, rest) = rest.into_future().wait().ok().unwrap();
  

  match item {
   None => println!("Got none"),
   Some(x) => println!("Got some value: {}", x),
  }

  let (item, rest) = rest.into_future().wait().ok().unwrap();
  match item {
    Some(x) => println!("got some interger: {}", x) ,
    None =>  println!("got none"),
  };

}

  

