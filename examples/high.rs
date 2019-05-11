use convert_base::Convert;

fn main () {
  let mut base = Convert::new(4000000000,700);
  let output = base.convert::<u32,u16>(&vec![
    3900000000, 3500004500, 3000000000, 2500000000,
    2000000000, 1500000000, 1000003300, 2500000000,
    3000000000, 3700050000, 2400000000, 1250000052
  ]);
  println!["{:?}", output];
}
