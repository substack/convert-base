extern crate convert_base;
use convert_base::Convert;

#[test]
fn convert () {
  assert_eq![
    Convert::new(256,4).convert(vec![97,98,99]),
    vec![1,0,2,1,2,0,2,1,3,0,2,1],
    "[97,98,99]@256 => @4"
  ];
  assert_eq![
    Convert::new(256,16).convert(vec![97,98,99]),
    vec![1,6,2,6,3,6],
    "[97,98,99]@256 => @16"
  ];
  assert_eq![
    Convert::new(4,256).convert(vec![1,0,2,1,2,0,2,1,3,0,2,1]),
    vec![97,98,99],
    "[1,0,2,1,2,0,2,1,3,0,2,1]@4 => @256"
  ];
}
