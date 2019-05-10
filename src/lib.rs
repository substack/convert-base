use std::ops::{Add,Div,Rem};

pub struct Convert { from: u64, to: u64 }

// custom trait because TryFrom is difficult:
pub trait FromU64 { fn from (n: u64) -> Self; }
impl FromU64 for u8 { fn from (n: u64) -> Self { n as u8 } }
impl FromU64 for u16 { fn from (n: u64) -> Self { n as u16 } }
impl FromU64 for u32 { fn from (n: u64) -> Self { n as u32 } }
impl FromU64 for u64 { fn from (n: u64) -> Self { n as u64 } }

impl Convert {
  pub fn new (from: u64, to: u64) -> Self {
    Convert { from, to }
  }
  pub fn convert<Input,Output> (&mut self, input: &Vec<Input>) -> Vec<Output>
  where Output: Copy+Into<u64>+From<u8>+FromU64
  +Add<Output,Output=Output>+Div<Output,Output=Output>+Rem<Output,Output=Output>,
  Input: Copy+Into<u64> {
    let len = input.len();
    let cap = len*ulog2(self.from)/ulog2(self.to);
    let mut output: Vec<Output> = Vec::with_capacity(cap);
    let mut base: Vec<Output> = vec![1u8.into()];
    let mut v0: Vec<Output> = vec![];
    for (i,x) in input.iter().enumerate() {
      Self::copy(&mut v0, &base);
      self.multiply_scalar_into(&mut v0, (*x).into());
      self.add_into(&mut output, &v0);
      if i+1 < input.len() {
        self.multiply_scalar_into(&mut base, self.from);
      }
    }
    output
  }
  fn copy<T> (dst: &mut Vec<T>, src: &Vec<T>) -> () where T: Copy {
    dst.clear();
    for x in src.iter() {
      dst.push(*x);
    }
  }
  fn multiply_scalar_into<T> (&self, dst: &mut Vec<T>, x: u64) -> ()
  where T: Copy+Into<u64>+FromU64 {
    let mut carry = 0u64;
    for i in 0..dst.len() {
      let res = dst[i].into() * x + carry;
      carry = res / (self.to as u64);
      dst[i] = FromU64::from(res % (self.to as u64));
    }
    while carry > 0 {
      dst.push(FromU64::from(carry % self.to));
      carry /= self.to;
    }
  }
  fn add_into<T> (&self, dst: &mut Vec<T>, src: &Vec<T>) -> ()
  where T: Copy+Into<u64>+FromU64
  +Add<T,Output=T>+Div<T,Output=T>+Rem<T,Output=T> {
    let mut carry = 0u64;
    for i in 0..dst.len().max(src.len()) {
      if i < src.len() && i < dst.len() {
        let res = src[i].into() + dst[i].into() + carry;
        carry = res / self.to;
        dst[i] = FromU64::from(res % self.to);
      } else if i < dst.len() {
        let res = dst[i].into() + carry;
        carry = res / self.to;
        dst[i] = FromU64::from(res % self.to);
      } else if i < src.len() {
        let res = src[i].into() + carry;
        carry = res / self.to;
        dst.push(FromU64::from(res % self.to));
      } else {
        let res = src[i].into() + carry;
        carry = res / self.to;
        dst.push(FromU64::from(res % self.to));
      }
    }
    while carry > 0 {
      let d = carry % self.to;
      dst.push(FromU64::from(d));
      carry /= self.to;
    }
  }
}

fn ulog2 (x: u64) -> usize { (63-x.leading_zeros()) as usize }
