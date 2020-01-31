# endian_codec

This crate helps serialize types as bytes and deserialize from bytes with a special
byte order. This crate can be used in [no_std] environment and has no external dependencies.

If you are looking for a small universal binary (de)serializer that works with
serde, look at [bincode].

Main features:
* A clean way to convert structures to endians and back
* Derive
* no_std and no external dependencies

### Examples
```rust
use endian_codec::{PackedSize, EncodeLE, DecodeLE};
// If you look at this structure without checking the documentation, you know it works with
// little-endian notation
#[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE)]
struct Version {
  major: u16,
  minor: u16,
  patch: u16
}

let mut buf = [0; Version::PACKED_LEN]; // From PackedSize
let test = Version { major: 0, minor: 21, patch: 37 };
// if you work with big- and little-endians, you will not mix them accidentally
test.encode_as_le_bytes(&mut buf);
let test_from_b = Version::decode_from_le_bytes(&buf);
assert_eq!(test, test_from_b);
```

There can be also a situation when you are forced to work with mixed-endians in one struct.
```rust
use endian_codec::{PackedSize, EncodeME};
// even if you only use derive EncodeME, you also need to have required traits in the scope.
use endian_codec::{EncodeLE, EncodeBE}; // for #[endian = "le/be"]

#[derive(PackedSize, EncodeME)]
// You work with a very old system and there are mixed-endians
// There will be only one format "le" or "little" in the next minor version.
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
// here we see me (mixed-endian), just look at the struct definition for details
req.encode_as_me_bytes(&mut buf);

```

#### Why another crate to handle endians?
* Easy byteorder-encoding structs with multiple fields and consistent encoding
* Learning how to create custom derives
* Making a cleaner API

#### There are a few other crates that deal with endians:
* [byteorder] -  Library for reading/writing numbers in big-endian and little-endian.
* [bytes] - Buf and BufMut traits that have methods to put and get primitives in the desired endian format.
* [simple_endian] - Instead of providing functions that convert - create types that store
variables in the desired endian format.
* [struct_deser] - Inspiration for this crate - but in a more clean and rusty way.


[bincode]:https://crates.io/crates/bincode
[byteorder]:https://crates.io/crates/byteorder
[bytes]:https://crates.io/crates/bytes
[simple_endian]:https://crates.io/crates/simple_endian
[struct_deser]:https://crates.io/crates/struct_deser
[no_std]:https://rust-embedded.github.io/book/intro/no-std.html

License: Apache-2.0 OR MIT
