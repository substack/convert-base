extern crate convert_base;
use convert_base::Convert;

#[test]
fn low_to_high_unaligned () {
  assert_eq![
    Convert::new(4,10).convert(vec![1,3,1,2]), // 1+4*3+16*1+64*2=157
    vec![7,5,1],
    "[1,3,1,2]@4 => @10"
  ];
  assert_eq![
    Convert::new(2,3).convert(vec![0,1,1,0,1]), // 2+4+16=22
    vec![1,1,2], // 1+3+18=22
    "[0,1,1,0,1]@2 => @3"
  ];
}

#[test]
fn high_to_low_unaligned () {
  assert_eq![
    Convert::new(10,4).convert(vec![7,5,1]),
    vec![1,3,1,2],
    "[7,5,1]@10 => @4"
  ];
  assert_eq![
    Convert::new(3,2).convert(vec![1,1,2]),
    vec![0,1,1,0,1],
    "[1,1,2]@3 => @2"
  ];
}
