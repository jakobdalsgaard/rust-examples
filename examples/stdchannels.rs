
use std::{time, thread};
use std::sync::mpsc::sync_channel;


pub fn main () {

  let (mut tx, mut rx) = sync_channel(0);

  thread::spawn(move || {
      thread::sleep(time::Duration::from_millis(3000));
      println!("sending 5 in 3 seconds");
      thread::sleep(time::Duration::from_millis(3000));
      tx.send(5).unwrap();
      println!("sending 7 in 3 seconds");
      thread::sleep(time::Duration::from_millis(3000));
      tx.send(7);
    });

  println!(" got {}", rx.recv().unwrap());
  println!(" got {}", rx.recv().unwrap());

}
