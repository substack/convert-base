extern crate convert_base;
use convert_base::Convert;

#[test]
fn low_to_high_unaligned_big () {
  assert_eq![
    Convert::new(16,300).convert(&vec![0xb,0x9,0x3,0xc]),
    vec![275,166],
    "c39b@16 => @300"
  ];
  assert_eq![
    Convert::new(300,500).convert(&vec![50,288,19,263,6]),
    vec![450,92,311,445],
    "@300 => @500"
  ];
}

#[test]
fn high_to_low_unaligned_big () {
  assert_eq![
    Convert::new(300,16).convert(&vec![275,166]),
    vec![0xb,0x9,0x3,0xc],
    "@300 => @16"
  ];
  assert_eq![
    Convert::new(500,300).convert(&vec![450,92,311,445]),
    vec![50,288,19,263,6],
    "@500 => @300"
  ];
}
