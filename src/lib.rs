pub struct Convert {
  from: usize,
  to: usize
}

type Input = u8;
type Output = u8;

impl Convert {
  pub fn new (from: usize, to: usize) -> Self {
    Convert { from, to }
  }
  pub fn convert (&mut self, input: Vec<Input>) -> Vec<Output> {
    let len = input.len();
    let cap = len*((self.from+self.to-1)/self.to);
    let mut output = Vec::with_capacity(cap);
    let mut bucket = 0u64;
    let mut p = 1u64;
    let m = self.from * self.to;
    let n_digits = ulog2(self.from as u64) / ulog2(self.to as u64);
    let aligned = self.to % self.from == 0 || self.from % self.to == 0;
    for (i,x) in input.iter().enumerate() {
      bucket += (*x as u64)*p;
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
}

fn ulog2 (x: u64) -> usize {
  (63-x.leading_zeros()) as usize
}
