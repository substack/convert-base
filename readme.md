# convert-base

convert the radix (base) of digits stored in a vector

* pure rust, no bigint deps or intermediate conversions
* designed around vectors of unsigned integer types, not strings

The algorithm is not yet clever enough to take advantage of base alignments, so
if you have aligned data, pre-process your inputs to use the alignment points.
Otherwise the implementation is `O(n^2)` on the length of the input `n`.

## examples

convert base 4 data stored in a `Vec<u8>` to base 500 data stored in a
`Vec<u16>`:

``` rust
use convert_base::Convert;

fn main () {
  let mut base = Convert::new(4,500);
  let output = base.convert::<u8,u16>(&vec![1,1,1,1,2,2,1,0,2,2,0,0,2,1]);
  println!["{:?}", output];
}
// output: [397, 150, 405]
```

or convert a `Vec<u32>` of base 4000000000 to a `Vec<u16>` of base 700:

``` rust
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
// output: [[300, 71, 255, 325, 23, 591, 267, 188, 488, 553, 124, 54, 422, 411,
//   116, 411, 85, 558, 4, 498, 384, 106, 465, 635, 75, 120, 226, 18, 634, 631,
//   116, 464, 111, 679, 17, 382, 67, 99, 208, 164, 8]
```

For input and output vectors, the least significant digit is at the
beginning of the array.

Internally, a u64 is used to hold intermediate calculations such as adding
two digits or performing carries. You will probably run out of precision if
you have an input or output base that is close to the maximum u64 value.

# api

``` rust
use convert_base::Convert;
```

## `let base = Convert::new(from: u64, to: u64)`

Create a new base conversion instance that converts between `from` and `to`.

## base.convert::<Input,Output>(input: Vec<Input>) : Vec<Output>

Perform the conversion on `input` which contains digits in base `self.from`. The
digits in the returned array will be in base the `self.to`. Make sure the
`Output` type has adequate capacity to hold digits in the output base
(`self.to`).

