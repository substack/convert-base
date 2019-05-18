use convert_base::Convert;

fn main() {
  let base = Convert::new(4, 500);
  let output = base.convert::<u8, u16>(&[1, 1, 1, 1, 2, 2, 1, 0, 2, 2, 0, 0, 2, 1]);
  println!["{:?}", output];
}
