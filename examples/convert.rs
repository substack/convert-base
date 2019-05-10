extern crate convert_base;
use convert_base::Convert;

fn main () {
  let mut base = Convert::new(256,4);
  let output: Vec<u64> = base.convert(&vec![97,98,99]);
  eprintln!["output={:?}", output];
}
