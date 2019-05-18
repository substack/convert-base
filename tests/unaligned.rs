use convert_base::Convert;

#[test]
fn low_to_high_unaligned() {
  assert_eq![
    // 1+4*3+16*1+64*2=157
    Convert::new(4, 10).convert::<u8, u8>(&[1, 3, 1, 2]),
    vec![7, 5, 1],
    "[1,3,1,2]@4 => @10"
  ];
  assert_eq![
    // 2+4+16=22
    Convert::new(2, 3).convert::<u8, u8>(&[0, 1, 1, 0, 1]),
    vec![1, 1, 2], // 1+3+18=22
    "[0,1,1,0,1]@2 => @3"
  ];
}

#[test]
fn high_to_low_unaligned() {
  assert_eq![
    Convert::new(10, 4).convert::<u8, u8>(&[7, 5, 1]),
    vec![1, 3, 1, 2],
    "[7,5,1]@10 => @4"
  ];
  assert_eq![
    Convert::new(3, 2).convert::<u8, u8>(&[1, 1, 2]),
    vec![0, 1, 1, 0, 1],
    "[1,1,2]@3 => @2"
  ];
}
