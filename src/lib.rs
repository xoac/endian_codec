//! This crate helps serialize types as bytes and deserialize from bytes with a special
//! byte order. This crate can be used in [no_std] environment and has no external dependencies.
//!
//! If you are looking for a small universal binary (de)serializer that works with
//! [serde], look at [bincode].
//!
//! Main features:
//! * A clean way to convert structures to endians and back
//! * Derive
//! * `no_std` and no external dependencies
//!
//! ## Examples
//! ```rust
//! use endian_codec::{PackedSize, EncodeLE, DecodeLE};
//! // If you look at this structure without checking the documentation, you know it works with
//! // little-endian notation
//! #[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE)]
//! struct Version {
//!   major: u16,
//!   minor: u16,
//!   patch: u16
//! }
//!
//! let mut buf = [0; Version::PACKED_LEN]; // From PackedSize
//! let test = Version { major: 0, minor: 21, patch: 37 };
//! // if you work with big- and little-endians, you will not mix them accidentally
//! test.encode_as_le_bytes(&mut buf);
//! let test_from_b = Version::decode_from_le_bytes(&buf);
//! assert_eq!(test, test_from_b);
//! ```
//!
//! There can be also a situation when you are forced to work with mixed-endians in one struct.
//! ```rust
//! use endian_codec::{PackedSize, EncodeME};
//! // even if you only use derive EncodeME, you also need to have required traits in the scope.
//! use endian_codec::{EncodeLE, EncodeBE}; // for #[endian = "le/be"]
//!
//! #[derive(PackedSize, EncodeME)]
//! // You work with a very old system and there are mixed-endians
//! // There will be only one format "le" or "little" in the next minor version.
//! struct Request {
//!   #[endian = "le"]
//!   cmd: u16,
//!   #[endian = "little"] // or #[endian = "le"]
//!   value: i64,
//!   #[endian = "big"] // or #[endian = "be"]
//!   timestamp: i128,
//! }
//!
//! let mut buf = [0; Request::PACKED_LEN];
//! let req = Request {
//!   cmd: 0x44,
//!   value: 74,
//!   timestamp: 0xFFFF_FFFF_0000_0000,
//! };
//! // here we see me (mixed-endian), just look at the struct definition for details
//! req.encode_as_me_bytes(&mut buf);
//!
//! ```
//!
//! ### Why another crate to handle endians?
//! * Easy byteorder-encoding structs with multiple fields and consistent encoding
//! * Learning how to create custom derives
//! * Making a cleaner API
//!
//! ### There are a few other crates that deal with endians:
//! * [byteorder] -  Library for reading/writing numbers in big-endian and little-endian.
//! * [bytes] - Buf and BufMut traits that have methods to put and get primitives in the desired endian format.
//! * [packed_struct] - Safe struct (un-) packing with bit-level control
//! * [simple_endian] - Instead of providing functions that convert - create types that store
//! variables in the desired endian format.
//! * [struct_deser] - Inspiration for this crate - but in a more clean and rusty way.
//!
//!
//! [bincode]:https://crates.io/crates/bincode
//! [byteorder]:https://crates.io/crates/byteorder
//! [bytes]:https://crates.io/crates/bytes
//! [packed_struct]:https://crates.io/crates/packed_struct
//! [simple_endian]:https://crates.io/crates/simple_endian
//! [struct_deser]:https://crates.io/crates/struct_deser
//! [no_std]:https://rust-embedded.github.io/book/intro/no-std.html
//! [serde]:https://crates.io/crates/serde

#![no_std]
#[cfg(feature = "endian_codec_derive")]
pub use endian_codec_derive::*;

/// Encoded as little-endian bytes.
pub trait EncodeLE: PackedSize {
    /// Borrow `self` and pack into `bytes` using little-endian representation.
    ///
    /// # Panics
    /// Panic if [PackedSize](PackedSize) represents a different size than `bytes` slice.
    ///
    fn encode_as_le_bytes(&self, bytes: &mut [u8]);
}

/// Encoded as big-endian bytes.
pub trait EncodeBE: PackedSize {
    /// Borrow `self` and pack into `bytes` using big-endian representation.
    ///
    /// # Panics
    ///
    /// Panic if [PackedSize](PackedSize) represents a different size than `bytes` slice.
    fn encode_as_be_bytes(&self, bytes: &mut [u8]);
}

/// Encode using mixed-endian bytes.
///
/// # Note
/// If you only use big-/little-endians, consider using [EncodeBE](EncodeBE) / [EncodeLE](EncodeLE) traits instead.
pub trait EncodeME: PackedSize {
    /// Borrow `self` and pack into `bytes` using mixed(custom)-endian representation.
    ///
    /// # Panics
    ///
    /// Panic if [PackedSize](PackedSize) represents a different size than `bytes` slice.
    fn encode_as_me_bytes(&self, bytes: &mut [u8]);
}

/// Decode from bytes stored as a little-endian.
pub trait DecodeLE: PackedSize {
    /// Read `bytes` slice packed as little-endian bytes and create `Self` from them
    ///
    /// # Panics
    ///
    /// Panic if [PackedSize](PackedSize) represents a different size than `bytes` slice.
    fn decode_from_le_bytes(bytes: &[u8]) -> Self;
}

/// Decode from bytes stored as a big-endian.
pub trait DecodeBE: PackedSize {
    /// Read `bytes` slice packed as big-endian bytes and create `Self` from them
    ///
    /// # Panics
    ///
    /// Panic if [PackedSize](PackedSize) represents a different size than `bytes` slice.
    fn decode_from_be_bytes(bytes: &[u8]) -> Self;
}

/// Decode from bytes stored as a mixed-endian.
///
/// # Note
/// If you only use big-/little-endians, consider using [DecodeBE](DecodeBE) / [DecodeLE](DecodeLE) traits instead.
pub trait DecodeME: PackedSize {
    /// Read `bytes` slice packed as mixed(custom)-endian bytes and create `Self` from them
    ///
    /// # Panics
    ///
    /// Panic if [PackedSize](PackedSize) represents a different size than `bytes` slice.
    fn decode_from_me_bytes(bytes: &[u8]) -> Self;
}

/// Represents size of a struct as packed bytes.
///
/// At this moment all settings with [repr](https://doc.rust-lang.org/nomicon/other-reprs.html)
/// attribute are ignored.
///
/// In other words if struct is marked as `repr(packed)` attribute, `std::mem::sizeof<T>()` should return the
/// same value as <T as PackedSize>::PACKED_LEN.
///
/// ```
/// // On a 64-bit machine, the size of struct A can be 16 bytes to make it more optimized for speed.
/// // but `PACKED_LEN` must be set to 12 bytes.
/// struct A {
///   p: i32,
///   v: i64,
/// }
/// ```
///
pub trait PackedSize {
    const PACKED_LEN: usize;
}

macro_rules! impl_codec_for_primitives {
    ($type:ty, $byte_len:expr) => {
        impl PackedSize for $type {
            const PACKED_LEN: usize = $byte_len;
        }

        impl EncodeLE for $type {
            #[inline]
            fn encode_as_le_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(&(self.to_le_bytes()))
            }
        }

        impl EncodeBE for $type {
            #[inline]
            fn encode_as_be_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(&(self.to_be_bytes()))
            }
        }

        impl DecodeLE for $type {
            #[inline]
            fn decode_from_le_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; $byte_len];
                arr.copy_from_slice(&bytes);
                Self::from_le_bytes(arr)
            }
        }

        impl DecodeBE for $type {
            #[inline]
            fn decode_from_be_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; $byte_len];
                arr.copy_from_slice(&bytes);
                Self::from_be_bytes(arr)
            }
        }
    };
}

impl_codec_for_primitives!(u8, 1);
impl_codec_for_primitives!(i8, 1);

impl EncodeME for u8 {
    #[inline]
    fn encode_as_me_bytes(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&(self.to_be_bytes()));
    }
}

impl DecodeME for u8 {
    #[inline]
    fn decode_from_me_bytes(bytes: &[u8]) -> Self {
        let mut arr = [0; 1];
        arr.copy_from_slice(bytes);
        Self::from_le_bytes(arr)
    }
}

impl_codec_for_primitives!(u16, 2);
impl_codec_for_primitives!(i16, 2);
impl_codec_for_primitives!(u32, 4);
impl_codec_for_primitives!(i32, 4);
impl_codec_for_primitives!(u64, 8);
impl_codec_for_primitives!(i64, 8);
impl_codec_for_primitives!(u128, 16);
impl_codec_for_primitives!(i128, 16);

macro_rules! impl_codec_for_array {
    ($type:ty, $size:expr) => {
        impl PackedSize for $type {
            const PACKED_LEN: usize = $size;
        }

        impl EncodeBE for $type {
            #[inline]
            fn encode_as_be_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self);
            }
        }

        impl EncodeLE for $type {
            #[inline]
            fn encode_as_le_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self);
            }
        }

        impl EncodeME for $type {
            #[inline]
            fn encode_as_me_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self);
            }
        }

        impl DecodeBE for $type {
            #[inline]
            fn decode_from_be_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::PACKED_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }

        impl DecodeLE for $type {
            #[inline]
            fn decode_from_le_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::PACKED_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }

        impl DecodeME for $type {
            #[inline]
            fn decode_from_me_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::PACKED_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }
    };
}

impl_codec_for_array!([u8; 1], 1);
impl_codec_for_array!([u8; 2], 2);
impl_codec_for_array!([u8; 3], 3);
impl_codec_for_array!([u8; 4], 4);
impl_codec_for_array!([u8; 5], 5);
impl_codec_for_array!([u8; 6], 6);
impl_codec_for_array!([u8; 7], 7);
impl_codec_for_array!([u8; 8], 8);
impl_codec_for_array!([u8; 9], 9);
impl_codec_for_array!([u8; 10], 10);
impl_codec_for_array!([u8; 11], 11);
impl_codec_for_array!([u8; 12], 12);
impl_codec_for_array!([u8; 13], 13);
impl_codec_for_array!([u8; 14], 14);
impl_codec_for_array!([u8; 15], 15);
impl_codec_for_array!([u8; 16], 16);
impl_codec_for_array!([u8; 17], 17);
impl_codec_for_array!([u8; 18], 18);
impl_codec_for_array!([u8; 19], 19);
impl_codec_for_array!([u8; 20], 20);
impl_codec_for_array!([u8; 21], 21);
impl_codec_for_array!([u8; 22], 22);
impl_codec_for_array!([u8; 23], 23);
impl_codec_for_array!([u8; 24], 24);
impl_codec_for_array!([u8; 25], 25);
impl_codec_for_array!([u8; 26], 26);
impl_codec_for_array!([u8; 27], 27);
impl_codec_for_array!([u8; 28], 28);
impl_codec_for_array!([u8; 29], 29);
impl_codec_for_array!([u8; 30], 30);
impl_codec_for_array!([u8; 31], 31);
impl_codec_for_array!([u8; 32], 32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_endian_size() {
        #[derive(PackedSize)]
        struct A {};
        assert_eq!(A::PACKED_LEN, 0);

        #[derive(PackedSize)]
        struct B {
            _a: u16,
        }
        assert_eq!(B::PACKED_LEN, 2);

        #[derive(PackedSize)]
        struct C {
            _a: u16,
            _b: u16,
        }
        assert_eq!(C::PACKED_LEN, 2 + 2);
    }

    #[test]
    fn derive_littlendian_serialize() {
        #[derive(PackedSize, EncodeLE)]
        struct Example {
            a: u16,
        }

        let t = Example { a: 5 };
        let mut b = [0; 2];
        t.encode_as_le_bytes(&mut b);
    }

    #[test]
    fn derive_bigendian_serialize() {
        #[derive(PackedSize, EncodeBE)]
        struct Example {
            a: u16,
        }

        let t = Example { a: 5 };
        let mut b = [0; 2];
        t.encode_as_be_bytes(&mut b);
    }

    #[test]
    fn derive_mixed_endian_serialize() {
        #[derive(PackedSize, EncodeME, Default)]
        struct Example {
            #[endian = "le"]
            a: u16,
            #[endian = "be"]
            b: u16,
            #[endian = "little"]
            aa: i16,
            #[endian = "big"]
            bb: i16,
        }

        let t = Example::default();
        let mut b = [0; 8];
        t.encode_as_me_bytes(&mut b);
    }

    #[test]
    fn derive_all_serialize() {
        #[derive(Default, PackedSize, EncodeLE, EncodeBE, EncodeME)]
        struct Example {
            #[endian = "be"]
            a: u16,
            b: [u8; 32],
        }

        let t = Example::default();
        let mut b = [0; 34];
        t.encode_as_me_bytes(&mut b);
        t.encode_as_be_bytes(&mut b);
        t.encode_as_le_bytes(&mut b);
    }

    #[test]
    fn derive_all() {
        #[derive(
            Default, PackedSize, EncodeLE, EncodeBE, EncodeME, DecodeLE, DecodeBE, DecodeME,
        )]
        struct Example {
            #[endian = "be"]
            a: u16,
        }

        let t = Example::default();
        let mut b = [0; 2];
        t.encode_as_me_bytes(&mut b);
        t.encode_as_be_bytes(&mut b);
        t.encode_as_le_bytes(&mut b);
    }

    #[test]
    fn test_codec_2bytes_primitives() {
        #[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE)]
        struct A {
            a: u16,
            b: i16,
        }

        let test = A { a: 0x2F, b: 0x2F00 };
        assert_eq!(A::PACKED_LEN, 4);
        let mut bytes = [0; A::PACKED_LEN];

        // LE
        test.encode_as_le_bytes(&mut bytes);
        assert_eq!([47, 0, 0, 47], bytes);

        let test_back = A::decode_from_le_bytes(&bytes);
        assert_eq!(test, test_back);

        //BE
        test.encode_as_be_bytes(&mut bytes);
        assert_eq!([0, 47, 47, 0], bytes);

        let test_back = A::decode_from_be_bytes(&bytes);
        assert_eq!(test, test_back);
    }

    #[test]
    fn test_codec_4bytes_primitives() {
        #[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE)]
        struct A {
            a: u32,
            b: i32,
        }

        let test = A {
            a: 0x2F,
            b: 0x2F000000,
        };
        assert_eq!(A::PACKED_LEN, 8);
        let mut bytes = [0; A::PACKED_LEN];

        // LE
        test.encode_as_le_bytes(&mut bytes);
        assert_eq!([47, 0, 0, 0, 0, 0, 0, 47], bytes);

        let test_back = A::decode_from_le_bytes(&bytes);
        assert_eq!(test, test_back);

        //BE
        test.encode_as_be_bytes(&mut bytes);
        assert_eq!([0, 0, 0, 47, 47, 0, 0, 0], bytes);

        let test_back = A::decode_from_be_bytes(&bytes);
        assert_eq!(test, test_back);
    }

    #[test]
    fn test_codec_8bytes_primitives() {
        #[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE)]
        struct A {
            a: u64,
            b: i64,
        }

        let test = A {
            a: 0x2F,
            b: 0x2F000000_00000000,
        };
        assert_eq!(A::PACKED_LEN, 16);
        let mut bytes = [0; A::PACKED_LEN];

        // LE
        test.encode_as_le_bytes(&mut bytes);
        assert_eq!([47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47], bytes);

        let test_back = A::decode_from_le_bytes(&bytes);
        assert_eq!(test, test_back);

        //BE
        test.encode_as_be_bytes(&mut bytes);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 47, 47, 0, 0, 0, 0, 0, 0, 0,], bytes);

        let test_back = A::decode_from_be_bytes(&bytes);
        assert_eq!(test, test_back);
    }

    #[test]
    fn test_codec_16bytes_primitives() {
        #[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE)]
        struct A {
            a: u128,
            b: i128,
        }

        let test = A {
            a: 0x2F,
            b: 0x2F000000_00000000_00000000_00000000,
        };
        assert_eq!(A::PACKED_LEN, 32);
        let mut bytes = [0; A::PACKED_LEN];

        // LE
        test.encode_as_le_bytes(&mut bytes);
        assert_eq!(
            [
                47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 47
            ],
            bytes
        );

        let test_back = A::decode_from_le_bytes(&bytes);
        assert_eq!(test, test_back);

        //BE
        test.encode_as_be_bytes(&mut bytes);
        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ],
            bytes
        );

        let test_back = A::decode_from_be_bytes(&bytes);
        assert_eq!(test, test_back);
    }

    /*
     This will not compile because EncodeME derive require A to implement EncodeME.
    #[test]
    fn derive_parameters() {
        #[derive(PackedSize, EncodeME)]
        struct Example<A> {
            #[endian = "big"]
            a: A,
            #[endian = "little"]
            be: u16,
        }
    }
    */
}
