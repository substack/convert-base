pub struct Convert {
  pub from: usize,
  pub to: usize
}

type Input = u8;
type Output = u8;

impl Convert {
  pub fn new (from: usize, to: usize) -> Self {
    Convert { from, to }
  }
  pub fn convert (&mut self, input: Vec<Input>) -> Vec<Output> {
    if self.from % self.to == 0 || self.to % self.from == 0 {
      self.convert_aligned(input)
    } else {
      self.convert_unaligned(input)
    }
  }
  fn convert_aligned (&mut self, input: Vec<Input>) -> Vec<Output> {
    let len = input.len();
    let cap = len*((self.from+self.to-1)/self.to);
    let mut output = Vec::with_capacity(cap);
    let mut bucket = 0u64;
    let mut p = 1u64;
    for (i,x) in input.iter().enumerate() {
      bucket += (*x as u64)*p;
      p *= self.from as u64;
      if p < self.to as u64 && i+1 != len { continue }
      p = 1u64;
      let n_digits = {
        let a = 63-(self.from as u64).leading_zeros();
        let b = 63-(self.to as u64).leading_zeros();
        a/b
      };
      let mut times = 0;
      while bucket > 0 || times == 0 {
        let d = bucket % (self.to as u64);
        output.push(d as Output);
        bucket /= self.to as u64;
        times += 1;
      }
      if i+1 < len {
        for _ in times..n_digits {
          output.push(0);
        }
      }
    }
    output
  }
  fn convert_unaligned (&mut self, _input: Vec<Input>) -> Vec<Output> {
    unimplemented![];
  }
}
