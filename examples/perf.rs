use convert_base::Convert;
use std::time::{Instant as Time};

fn main () {
  let mut base = Convert::new(243,9);
  let start = Time::now();
  let input: Vec<u8> = (0..5_000_000).map(|i| (i%243u64) as u8).collect();
  let output = base.convert::<u8,u8>(&input);
  println!["{}",start.elapsed().as_secs_f64()];
  println!["{} items", output.len()];
}
