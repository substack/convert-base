pub struct Convert {
  from: u64,
  to: u64
}

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
    for (i,x) in input.iter().enumerate() {
      Self::copy(&mut v0, &base);
      self.multiply_scalar_into(&mut v0, *x);
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
  fn multiply_scalar_into (&self, dst: &mut Vec<T>, x: T) -> () {
    let mut carry: T = 0;
    for i in 0..dst.len() {
      let res: u64 = dst[i] * x + carry;
      carry = res / self.to;
      dst[i] = res % self.to;
    }
    while carry > 0 {
      dst.push(carry % self.to);
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
}

fn ulog2 (x: u64) -> usize {
  (63-x.leading_zeros()) as usize
}
