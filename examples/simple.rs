use std::thread;
use std::sync::mpsc::channel;

// Create a simple streaming channel
pub fn main () {
let (tx, rx) = channel();
thread::spawn(move|| {
    tx.send(10).unwrap();
});
println!("got {}", rx.recv().unwrap());
}



