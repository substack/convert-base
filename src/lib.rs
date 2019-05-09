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
    let cap = input.len()*((self.from+self.to-1)/self.to);
    let mut output = Vec::with_capacity(cap);
    let mut bucket = 0u64;
    let mut p = 1u64;
    for x in input.iter() {
      bucket += (*x as u64)*p;
      p *= self.from as u64;
      if p < self.to as u64 { continue }
      p = 1u64;
      while bucket > 0 {
        let d = bucket % (self.to as u64);
        output.push(d as Output);
        bucket /= self.to as u64;
      }
    }
    output
  }
  fn convert_unaligned (&mut self, _input: Vec<Input>) -> Vec<Output> {
    unimplemented![];
  }
}
