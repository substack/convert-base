//! convert the radix (base) of digits stored in a vector
//!
//! * pure rust, no bigint deps or intermediate conversions
//! * designed around vectors of unsigned integer types, not strings
//!
//! The algorithm is not yet clever enough to take advantage of base alignments, so
//! if you have aligned data, pre-process your inputs to use the alignment points.
//! Otherwise the implementation is `O(n^2)` on the length of the input `n`.
//!
//! ## examples
//!
//! convert base 4 data stored in a `Vec<u8>` to base 500 data stored in a
//! `Vec<u16>`:
//!
//! ``` rust
//! extern crate convert_base;
//! use convert_base::Convert;
//!
//! fn main () {
//!   let mut base = Convert::new(4,500);
//!   let output = base.convert::<u8,u16>(&vec![1,1,1,1,2,2,1,0,2,2,0,0,2,1]);
//!   println!["{:?}", output];
//! }
//! // output: [397, 150, 405]
//! ```
//!
//! or convert a `Vec<u32>` of base 4068505555 to a `Vec<u16>` of base 700:
//!
//! ``` rust
//! extern crate convert_base;
//! use convert_base::Convert;
//!
//! fn main () {
//!   let mut base = Convert::new(4068505555,700);
//!   let output = base.convert::<u32,u16>(&vec![
//!     4000000000, 3500000000, 3000000000, 2500000000,
//!     2000000000, 1500000000, 1000000000, 2500000000,
//!     3000000000, 4100000000, 2400000000, 1250000052
//!   ]);
//!   println!["{:?}", output];
//! }
//! // output: [100, 182, 419, 476, 133, 314, 42, 217, 163, 481, 171, 122, 647,
//! //   644, 67, 388, 566, 168, 16, 583, 198, 688, 404, 264, 686, 110, 694,
//! //   444, 48, 338, 427, 611, 92, 564, 101, 622, 369, 637, 119, 648, 9]
//! ```
//!
//! For input and output vectors, the least significant digit is at the
//! beginning of the array.
//!
//! Internally, a u64 is used to hold intermediate calculations such as adding
//! two digits or performing carries. You will probably run out of precision if
//! you have an input or output base that is close to the maximum u64 value.

use std::ops::{Add,Div,Rem};

pub struct Convert { from: u64, to: u64 }

impl Convert {
  /// Create a new converter with `from` and `to` bases.
  pub fn new (from: u64, to: u64) -> Self {
    Convert { from, to }
  }
  /// Perform the conversion on `input` which contains digits in base
  /// `self.from`. You should specify the `Output` type so that the target base
  /// (`self.to`) fits. This isn't checked for you.
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
      // TODO: aligned bases can reset base value
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

// custom trait because TryFrom is difficult:
pub trait FromU64 { fn from (n: u64) -> Self; }
impl FromU64 for u8 { fn from (n: u64) -> Self { n as u8 } }
impl FromU64 for u16 { fn from (n: u64) -> Self { n as u16 } }
impl FromU64 for u32 { fn from (n: u64) -> Self { n as u32 } }
impl FromU64 for u64 { fn from (n: u64) -> Self { n as u64 } }
