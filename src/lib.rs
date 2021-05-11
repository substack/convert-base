//! Convert the radix (base) of digits stored in a vector.
//!
//! * pure rust, no bigint deps or intermediate conversions
//! * designed around vectors of unsigned integer types, not strings
//! * very fast on large vectors when bases are aligned
//!   (see performance section below)
//!
//! # examples
//!
//! convert base 4 data stored in a `Vec<u8>` to base 500 data stored in a
//! `Vec<u16>`:
//!
//! ``` rust
//! use convert_base::Convert;
//!
//! fn main () {
//!   let mut base = Convert::new(4,500);
//!   let output = base.convert::<u8,u16>(&vec![1,1,1,1,2,2,1,0,2,2,0,0,2,1]);
//!   assert_eq!{output, vec![397, 150, 405]};
//! }
//! ```
//!
//! or convert a `Vec<u32>` of base `4_000_000_000` to a `Vec<u16>` of base 700:
//!
//! ``` rust
//! use convert_base::Convert;
//!
//! fn main () {
//!   let mut base = Convert::new(4_000_000_000,700);
//!   let output = base.convert::<u32,u16>(&vec![
//!     3_900_000_000, 3_500_004_500, 3_000_000_000, 2_500_000_000,
//!     2_000_000_000, 1_500_000_000, 1_000_003_300, 2_500_000_000,
//!     3_000_000_000, 3_700_050_000, 2_400_000_000, 1_250_000_052
//!   ]);
//!   assert_eq![output, vec!{
//!     300, 71, 255, 325, 23, 591, 267, 188, 488, 553, 124, 54, 422, 411, 116,
//!     411, 85, 558, 4, 498, 384, 106, 465, 635, 75, 120, 226, 18, 634, 631,
//!     116, 464, 111, 679, 17, 382, 67, 99, 208, 164, 8
//!   }];
//! }
//! ```
//!
//! For input and output vectors, the least significant digit is at the
//! beginning of the array.
//!
//! Internally, a u64 is used to hold intermediate calculations such as adding
//! two digits or performing carries. You will probably run out of precision if
//! you have an input or output base that is close to the maximum u64 value.
//!
//! # performance
//!
//! When the bases are "aligned" the base conversion can be very fast. But
//! converting long vectors between unaligned bases can be very slow.
//!
//! Two bases are "aligned" when two integers `a` and `b` satisfy the equation
//! `base1.pow(a) == base2.pow(b)`. This ratio of `a:b` describes how bases can
//! cleanly overlap. For example 3 digits in base 256 corresponds exactly to 4
//! digits in base 64. Or 2 digits in base 243 corresponds exactly to 10 digits
//! in base 3 (because `243.pow(2) == 3.pow(10)`).
//!
//! On this old 2014 laptop, converting `5_000` digits:
//!
//! * from base 243 to base 9 in 0.00234 seconds
//! * from base 243 to base 10 in 1.26 seconds
//!
//! and converting `50_000` digits:
//!
//! * from base 243 to base 9 in 0.0149 seconds
//! * from base 243 to base 10 in 125.3 seconds

use std::ops::{Add,Div,Rem};

/// Convert the radix (base) of digits stored in a vector.
pub struct Convert {
  from: u64,
  to: u64,
  ratio: (usize,usize)
}

impl Convert {
  /// Create a new converter with `from` and `to` bases.
  pub fn new (from: u64, to: u64) -> Self {
    let mut ratio = (0,0);
    if from % to == 0 || to % from == 0 {
      let max_i = 128 / ulog2(to.max(from));
      let mut j = 0;
      let mut k = 0;
      let f = from as u128;
      let t = to as u128;
      for i in 0..max_i {
        let f_j = f.pow(j);
        let t_k = t.pow(k);
        if i > 0 && f_j == t_k {
          ratio.0 = j as usize;
          ratio.1 = k as usize;
          break
        } else if f_j < t_k || (i == 0 && from > to) {
          j += 1
        } else { k+=1 }
      }
    }
    Convert { from, to, ratio }
  }
  /// Create a new converter but don't test for alignment.
  pub fn new_unaligned (from: u64, to: u64) -> Self {
    Convert { from, to, ratio: (0,0) }
  }
  /// Perform the conversion on `input` which contains digits in base
  /// `self.from`. You should specify the `Output` type so that the target base
  /// (`self.to`) fits. There are no checks to ensure the `Output` type has
  /// room.
  ///
  /// For input and output vectors, the least significant digit is at the
  /// beginning of the array.
  pub fn convert<Input,Output> (&mut self, input: &[Input]) -> Vec<Output>
  where Output: Copy+Into<u64>+From<u8>+FromU64
  +Add<Output,Output=Output>+Div<Output,Output=Output>+Rem<Output,Output=Output>,
  Input: Copy+Into<u64> {
    let len = input.len();
    let cap = len*ulog2(self.from)/ulog2(self.to);
    let mut output: Vec<Output> = Vec::with_capacity(cap);
    let mut base: Vec<Output> = vec![1u8.into()];
    let mut v0: Vec<Output> = vec![];
    let step = self.ratio.0;
    let mut offset = 0;
    for (i,x) in input.iter().enumerate() {
      v0.clear();
      v0.extend(&base);
      self.multiply_scalar_into(&mut v0, (*x).into());
      self.add_into(&mut output, &v0, offset);
      if i+1 < input.len() {
        self.multiply_scalar_into(&mut base, self.from);
      }
      if step > 0 && i%step == step-1 {
        base.clear();
        base.push(1u8.into());
        offset += self.ratio.1;
      }
    }
    output
  }
  fn multiply_scalar_into<T> (&self, dst: &mut Vec<T>, x: u64) -> ()
  where T: Copy+Into<u64>+FromU64 {
    let mut carry = 0u64;
    for i in 0..dst.len() {
      let res = dst[i].into() * x + carry;
      carry = res / self.to;
      dst[i] = FromU64::from(res % (self.to as u64));
    }
    while carry > 0 {
      dst.push(FromU64::from(carry % self.to));
      carry /= self.to;
    }
  }
  fn add_into<T> (&self, dst: &mut Vec<T>, src: &Vec<T>, offset: usize) -> ()
  where T: Copy+Into<u64>+FromU64
  +Add<T,Output=T>+Div<T,Output=T>+Rem<T,Output=T> {
    let mut carry = 0u64;
    let mut i = 0;
    while dst.len().max(offset)-offset < src.len() {
      dst.push(FromU64::from(0));
    }
    loop {
      let j = i + offset;
      if i < src.len() && j < dst.len() {
        let res = src[i].into() + dst[j].into() + carry;
        carry = res / self.to;
        dst[j] = FromU64::from(res % self.to);
      } else if j < dst.len() {
        let res = dst[j].into() + carry;
        carry = res / self.to;
        dst[j] = FromU64::from(res % self.to);
      } else if i < src.len() {
        let res = src[i].into() + carry;
        carry = res / self.to;
        dst.push(FromU64::from(res % self.to));
      } else if carry > 0 {
        let res = carry;
        carry = res / self.to;
        dst.push(FromU64::from(res % self.to));
      } else {
        break;
      }
      i += 1;
    }
  }
}

fn ulog2 (x: u64) -> usize { (63-x.leading_zeros()) as usize }

// custom trait because TryFrom is difficult:
#[doc(hidden)]
pub trait FromU64 { fn from (n: u64) -> Self; }
impl FromU64 for u8 { fn from (n: u64) -> Self { n as u8 } }
impl FromU64 for u16 { fn from (n: u64) -> Self { n as u16 } }
impl FromU64 for u32 { fn from (n: u64) -> Self { n as u32 } }
impl FromU64 for u64 { fn from (n: u64) -> Self { n as u64 } }
