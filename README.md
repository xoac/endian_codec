# endian_codec

This crate is to help serialize types as bytes and deserialize from bytes with special
byte order. This crate can be used in [no_std] environment and has no external dependencies.

If you are looking for small universal binary (de)serializer that works with
serde look at [bincode].

What this crate provide:
* General traits that in clean way adding endian conversions.
* Derive
* no_std and no external dependencies

### Examples
```rust
use endian_codec::{PackedSize, EncodeLE, DecodeLE};
// If you look at this structure you know without documentation it works with little endian
// notation
#[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE)]
struct Version {
  major: u16,
  minor: u16,
  patch: u16
}

let mut buf = [0; Version::PACKED_LEN]; // From PackedSize
let test = Version { major: 0, minor: 21, patch: 37 };
// if you work with big and little endian you will not mix them accidentally
test.encode_as_le_bytes(&mut buf);
let test_from_b = Version::decode_from_le_bytes(&buf);
assert_eq!(test, test_from_b);
```

There can be also situation when you are forced to work with mixed endian in one struct.
```rust
use endian_codec::{PackedSize, EncodeME};
// even if you use only derive EncodeME you also need used traits in scope.
use endian_codec::{EncodeLE, EncodeBE}; // for #[endian = "le/be"]

#[derive(PackedSize, EncodeME)]
// You work with very old system and there are mixed endianness
// There will be only one format "le" or "little" in next minor version.
struct Request {
  #[endian = "le"]
  cmd: u16,
  #[endian = "little"] // or #[endian = "le"]
  value: i64,
  #[endian = "big"] // or #[endian = "be"]
  timestamp: i128,
}

let mut buf = [0; Request::PACKED_LEN];
let req = Request {
  cmd: 0x44,
  value: 74,
  timestamp: 0xFFFF_FFFF_0000_0000,
};
// here we see me (mixed endian) just look on struct definition for deatels
req.encode_as_me_bytes(&mut buf);

```

#### Why another crate to handle endian?
* easily byteorder-encoding structs with multiple fields in a consistent encoding
* learn how to create custom derives
* make a cleaner API

#### There are few other crates that deal with endian:
* [byteorder] -  Library for reading/writing numbers in big-endian and little-endian.
* [bytes] - Buf and BufMut traits have methods to put and get primitives in wanted endian.
* [simple_endian] - Instead of provide functions that converts - create types that store
variables in wanted endian.
* [struct_deser] - Inspiration for this crate - but in more clean and rusty way.


[bincode]:https://crates.io/crates/bincode
[byteorder]:https://crates.io/crates/byteorder
[bytes]:https://crates.io/crates/bytes
[simple_endian]:https://crates.io/crates/simple_endian
[struct_deser]:https://crates.io/crates/struct_deser
[no_std]:https://rust-embedded.github.io/book/intro/no-std.html
