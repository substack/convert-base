# convert-base

Convert the radix (base) of digits stored in a vector.

* pure rust, no bigint deps or intermediate conversions
* designed around vectors of unsigned integer types, not strings
* very fast on large vectors when bases are aligned
  (see performance section below)

# examples

convert base 4 data stored in a `Vec<u8>` to base 500 data stored in a
`Vec<u16>`:

``` rust
use convert_base::Convert;

fn main () {
  let mut base = Convert::new(4,500);
  let output = base.convert::<u8,u16>(&vec![1,1,1,1,2,2,1,0,2,2,0,0,2,1]);
  assert_eq!{output, vec![397, 150, 405]};
}
```

or convert a `Vec<u32>` of base `4_000_000_000` to a `Vec<u16>` of base 700:

``` rust
use convert_base::Convert;

fn main () {
  let mut base = Convert::new(4_000_000_000,700);
  let output = base.convert::<u32,u16>(&vec![
    3_900_000_000, 3_500_004_500, 3_000_000_000, 2_500_000_000,
    2_000_000_000, 1_500_000_000, 1_000_003_300, 2_500_000_000,
    3_000_000_000, 3_700_050_000, 2_400_000_000, 1_250_000_052
  ]);
  assert_eq![output, vec!{
    300, 71, 255, 325, 23, 591, 267, 188, 488, 553, 124, 54, 422, 411, 116,
    411, 85, 558, 4, 498, 384, 106, 465, 635, 75, 120, 226, 18, 634, 631,
    116, 464, 111, 679, 17, 382, 67, 99, 208, 164, 8
  }];
}
```

For input and output vectors, the least significant digit is at the
beginning of the array.

Internally, a u64 is used to hold intermediate calculations such as adding
two digits or performing carries. You will probably run out of precision if
you have an input or output base that is close to the maximum u64 value.

# performance

When the bases are "aligned" the base conversion can be very fast. But
converting long vectors between unaligned bases can be very slow.

Two bases are "aligned" when two integers `a` and `b` satisfy the equation
`base1.pow(a) == base2.pow(b)`. This ratio of `a:b` describes how bases can
cleanly overlap. For example 3 digits in base 256 corresponds exactly to 4
digits in base 64. Or 2 digits in base 243 corresponds exactly to 10 digits
in base 3 (because `243.pow(2) == 3.pow(10)`).

On this old 2014 laptop, converting `5_000` digits:

* from base 243 to base 9 in 0.00234 seconds
* from base 243 to base 10 in 1.26 seconds

and converting `50_000` digits:

* from base 243 to base 9 in 0.0149 seconds
* from base 243 to base 10 in 125.3 seconds

# api

``` rust
use convert_base::Convert;
```

## `let base = Convert::new(from: u64, to: u64)`

Create a new base conversion instance that converts between `from` and `to`.

## `let base = Convert::new_unaligned(from: u64, to: u64)`

Create a new base conversion instance without checking for base alignment.

## `base.convert::<Input,Output>(input: Vec<Input>) : Vec<Output>`

Perform the conversion on `input` which contains digits in base `self.from`. The
digits in the returned array will be in base the `self.to`. Make sure the
`Output` type has adequate capacity to hold digits in the output base
(`self.to`).

