pub struct Convert {
  from: usize,
  to: usize
}

pub trait Num {
  fn to_u64 (&self) -> u64;
  fn from_u64 (n: u64) -> Self;
}
macro_rules! impl_num {
  ($T:ty) => {
    impl Num for $T {
      fn to_u64 (&self) -> u64 { *self as u64 }
      fn from_u64 (n: u64) -> Self { n as Self }
    }
  };
  ($H:ty,$($T:ty),+) => {
    impl_num![$H];
    impl_num![$($T),+];
  };
}
impl_num![u8,u16,u32,u64,u128,i8,i16,i32,i64,i128];

impl Convert {
  pub fn new (from: usize, to: usize) -> Self {
    Convert { from, to }
  }
  pub fn convert<I,O> (&mut self, input: &Vec<I>) -> Vec<O>
  where I: Num, O: Num {
    let len = input.len();
    let cap = len*ulog2(self.from as u64)/ulog2(self.to as u64);
    let mut output: Vec<O> = Vec::with_capacity(cap);
    let mut bucket = 0u64;
    let mut p = 1u64;
    let g = gcd(self.from, self.to);
    let m = self.from / g * self.to / g;
    let n_digits = ulog2(self.from as u64) / ulog2(self.to as u64);
    let aligned = self.to % self.from == 0 || self.from % self.to == 0;
    for (i,x) in input.iter().enumerate() {
      let b: u64 = x.to_u64();
      bucket += b * p;
      p *= self.from as u64;
      if aligned {
        if p < self.to as u64 && i+1 != len { continue }
      } else {
        if i % m != m-1 && i+1 != len { continue }
      }
      p = 1u64;
      let mut times = 0;
      while bucket > 0 || times == 0 {
        let d = bucket % (self.to as u64);
        output.push(Num::from_u64(d));
        bucket /= self.to as u64;
        times += 1;
      }
      if i+1 < len {
        for _ in times..n_digits {
          output.push(Num::from_u64(0));
        }
      }
    }
    output
  }
}

fn ulog2 (x: u64) -> usize {
  (63-x.leading_zeros()) as usize
}

fn gcd (a: usize, b: usize) -> usize {
  if b == 0 { a }
  else { gcd(b, a % b) }
}
