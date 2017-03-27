use std::thread;
use std::sync::mpsc::channel;

// Create a simple streaming channel
pub fn main () {
  let mut c = 1u64;
  println!("{:06x}", &c);

  c = c + 64;
  println!("{:06x}", &c);  

}



