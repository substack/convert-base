extern crate convert_base;
use convert_base::Convert;

#[test]
fn high_to_low_aligned () {
  assert_eq![
    Convert::new(256,4).convert(&vec![97,98,99]),
    vec![1,0,2,1,2,0,2,1,3,0,2,1],
    "[97,98,99]@256 => @4"
  ];
  assert_eq![
    Convert::new(256,16).convert(&vec![97,98,99]),
    vec![1,6,2,6,3,6],
    "[97,98,99]@256 => @16"
  ];
  assert_eq![
    Convert::new(256,2).convert(&vec![97,98,99]),
    vec![
      1,0, 0,0, 0,1, 1,0,
      0,1, 0,0, 0,1, 1,0,
      1,1, 0,0, 0,1, 1
    ],
    "[97,98,99]@256 => @2"
  ];
  assert_eq![
    Convert::new(4,2).convert(&vec![1,0,2,1,2,0,2,1,3,0,2,1]),
    vec![
      1,0, 0,0, 0,1, 1,0,
      0,1, 0,0, 0,1, 1,0,
      1,1, 0,0, 0,1, 1
    ],
    "[1,0,2,1,2,0,2,1,3,0,2,1]@4 => @2"
  ];
}

#[test]
fn low_to_high_aligned () {
  assert_eq![
    Convert::new(4,256).convert(&vec![1,0,2,1,2,0,2,1,3,0,2,1]),
    vec![97,98,99],
    "[1,0,2,1,2,0,2,1,3,0,2,1]@4 => @256"
  ];
  assert_eq![
    Convert::new(16,256).convert(&vec![1,6,2,6,3,6]),
    vec![97,98,99],
    "[1,6,2,6,3,6]@16 => @256"
  ];
  assert_eq![
    Convert::new(4,16).convert(&vec![1,0,2,1,2,0,2,1,3,0,2,1]),
    vec![1,6,2,6,3,6],
    "[1,0,2,1,2,0,2,1,3,0,2,1]@4 => @16"
  ];
  assert_eq![
    Convert::new(2,4).convert(&vec![
      1,0, 0,0, 0,1, 1,0,
      0,1, 0,0, 0,1, 1,0,
      1,1, 0,0, 0,1, 1
    ]),
    vec![1,0,2,1,2,0,2,1,3,0,2,1],
    "[10000110 01000110 1100011]@2 => @4"
  ];
}

#[test]
fn edge_cases_aligned () {
  assert_eq![
    Convert::new(2,4).convert(&vec![]),
    vec![],
    "low empty to high empty"
  ];
  assert_eq![
    Convert::new(4,2).convert(&vec![]),
    vec![],
    "high empty to low empty"
  ];
  assert_eq![
    Convert::new(2,4).convert(&vec![1]),
    vec![1],
    "low to high single item"
  ];
  assert_eq![
    Convert::new(4,2).convert(&vec![1]),
    vec![1],
    "high to low single"
  ];
  assert_eq![
    Convert::new(4,2).convert(&vec![0,1]),
    vec![0,0,1],
    "high to low double to triple"
  ];
  assert_eq![
    Convert::new(2,4).convert(&vec![0,0,1]),
    vec![0,1],
    "low to high triple to double"
  ];
}
