use std::ops::Add;

fn addthis<T:Add>(a: T, b: T) -> T::Output {
  a + b
}


#[derive(Debug)]
pub enum Cycle {
  UNO,
  DOS,
  TRES,
}

impl Add for Cycle {
  type Output = Cycle;

  fn add(self, other: Cycle) -> Self::Output {
    match (self, other) {
     (Cycle::UNO, Cycle::UNO) => Cycle::DOS,
     (Cycle::UNO, Cycle::DOS) => Cycle::TRES,
     (Cycle::DOS, Cycle::UNO) => Cycle::TRES,
     (Cycle::DOS, Cycle::DOS) => Cycle::UNO,
     (Cycle::TRES, x) => x,
     (x, Cycle::TRES) => x,
   }
     
  }
}

fn main() {
  println!("1 + 2 is: {}", addthis(1u16, 2u16));
  println!("0.5 + 0.2 is: {}", addthis(0.5f32, 0.2f32));
  println!("DOS + DOS is: {:?}", addthis(Cycle::DOS, Cycle::DOS));
}

