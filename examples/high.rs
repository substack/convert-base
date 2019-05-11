use convert_base::Convert;

fn main () {
  let mut base = Convert::new(4068505555,700);
  let output = base.convert::<u32,u16>(&vec![
    4000000000, 3500000000, 3000000000, 2500000000,
    2000000000, 1500000000, 1000000000, 2500000000,
    3000000000, 4100000000, 2400000000, 1250000052
  ]);
  println!["{:?}", output];
}
