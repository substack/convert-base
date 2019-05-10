pub struct Convert {
  from: u64,
  to: u64
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

type T = u64;

impl Convert {
  pub fn new (from: u64, to: u64) -> Self {
    Convert { from, to }
  }
  pub fn convert (&mut self, input: &Vec<T>) -> Vec<T> {
    let len = input.len();
    let cap = len*ulog2(self.from)/ulog2(self.to);
    let mut output: Vec<T> = Vec::with_capacity(cap);
    let mut base: Vec<T> = vec![1];
    let mut v0: Vec<T> = vec![];
    let mut v1: Vec<T> = vec![];
    for (i,x) in input.iter().enumerate() {
      Self::copy(&mut v0, &base);
      self.multiply_scalar(&mut base, &v0, self.from);
      self.multiply_scalar(&mut v0, &base, *x);
      eprintln!["add {:?} into {:?}", v0, output];
      self.add_into(&mut output, &v0);
    }
    output
  }
  fn copy<T> (dst: &mut Vec<T>, src: &Vec<T>) -> () where T: Copy {
    dst.clear();
    for x in src.iter() {
      dst.push(*x);
    }
  }
  fn multiply_scalar (&self, dst: &mut Vec<T>, src: &Vec<T>, x: T) -> () {
    let mut carry: T = 0;
    for i in 0..dst.len() {
      let res: u64 = dst[i] * x + carry;
      carry = res / self.to;
      // todo: push carry forward
      dst[i] = res % self.to;
    }
    while carry > 0 {
      let d = carry % self.to;
      dst.push(d);
      carry /= self.to;
    }
  }
  fn add_into (&self, dst: &mut Vec<T>, src: &Vec<T>) -> () {
    let mut carry: T = 0;
    for i in 0..dst.len().max(src.len()) {
      if i < src.len() && i < dst.len() {
        let res = src[i] + dst[i] + carry;
        carry = res / self.to;
        dst[i] = res % self.to;
      } else if i < dst.len() {
        let res = dst[i] + carry;
        carry = res / self.to;
        dst[i] = res % self.to;
      } else if i < src.len() {
        let res = src[i] + carry;
        carry = res / self.to;
        dst.push(res % self.to);
      } else {
        let res = src[i] + carry;
        carry = res / self.to;
        dst.push(res % self.to);
      }
    }
    while carry > 0 {
      let d = carry % self.to;
      dst.push(d);
      carry /= self.to;
    }
  }
  /*
  pub fn convert<Input,Output> (&mut self, input: &Vec<Input>) -> Vec<Output>
  where Input: Num, Output: Num {
    let len = input.len();
    let cap = len*ulog2(self.from as u64)/ulog2(self.to as u64);
    let mut output: Vec<Output> = Vec::with_capacity(cap);
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
  */
}

fn ulog2 (x: u64) -> usize {
  (63-x.leading_zeros()) as usize
}

/*
fn gcd (a: usize, b: usize) -> usize {
  if b == 0 { a }
  else { gcd(b, a % b) }
}
*/
