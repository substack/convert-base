extern crate convert_base;
use convert_base::Convert;

fn main () {
  let mut base = Convert::new(256,4);
  let output = base.convert::<u8,u8>(&vec![97,98,99]);
  eprintln!["output={:?}", output];
}
