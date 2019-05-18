use convert_base::Convert;

#[test]
fn low_to_high_unaligned_big() {
  assert_eq![
    Convert::new(16, 300).convert::<u8, u16>(&[0xb, 0x9, 0x3, 0xc]),
    vec![275, 166],
    "c39b@16 => @300"
  ];
  assert_eq![
    Convert::new(300, 500).convert::<u16, u16>(&[50, 288, 19, 263, 6]),
    vec![450, 92, 311, 445],
    "@300 => @500"
  ];
  assert_eq![
    Convert::new(10, 36_052_000).convert::<u8, u32>(&[
      1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9, 8, 7,
      6, 5, 4, 3, 2, 1
    ]),
    vec![8_242_321, 7_535_681, 4_301_677, 10_786_674, 730_798]
  ];
}

#[test]
fn high_to_low_unaligned_big() {
  assert_eq![
    Convert::new(300, 16).convert::<u16, u8>(&[275, 166]),
    vec![0xb, 0x9, 0x3, 0xc],
    "@300 => @16"
  ];
  assert_eq![
    Convert::new(500, 300).convert::<u16, u16>(&[450, 92, 311, 445]),
    vec![50, 288, 19, 263, 6],
    "@500 => @300"
  ];
  assert_eq![
    Convert::new(36_052_000, 10)
      .convert::<u32, u8>(&[8_242_321, 7_535_681, 4_301_677, 10_786_674, 730_798]),
    vec![
      1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9, 8, 7,
      6, 5, 4, 3, 2, 1
    ]
  ];
}
