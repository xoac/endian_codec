//! This crate is to help serialize types as bytes and deserialize from bytes with special
//! byte order. This crate can be used in [no_std] environment and has no external dependencies.
//!
//! If you are looking for small universal binary (de)serializer that works with
//! serde look at [bincode].
//!
//! What this crate provide:
//! * General traits that in clean way adding endian conversions.
//! * Derive
//! * no_std and no external dependencies
//!
//! ## Examples
//! ```rust
//! use endian_serde::{EndianSize, LittleEndianSerialize, LittleEndianDeserialize};
//! // If you look at this structure you know without documentation it works with little endian
//! // notation
//! #[derive(Debug, PartialEq, Eq, EndianSize, LittleEndianSerialize, LittleEndianDeserialize)]
//! struct Version {
//!   major: u16,
//!   minor: u16,
//!   patch: u16
//! }
//!
//! let mut buf = [0; Version::BYTES_LEN]; // From EndianSize
//! let test = Version { major: 0, minor: 21, patch: 37 };
//! // if you work with big and little endian you will not mix them accidentally
//! test.serialize_as_le_bytes(&mut buf);
//! let test_from_b = Version::deserialize_from_le_bytes(&buf);
//! assert_eq!(test, test_from_b);
//! ```
//!
//! There can be also situation when you are forced to work with mixed endian in one struct.
//! ```rust
//! use endian_serde::{EndianSize, MixedEndianSerialize};
//! // even if you use only derive MixedEndianSerialize you also need used traits in scope.
//! use endian_serde::{LittleEndianSerialize, BigEndianSerialize}; // for #[endian = "le/be"]
//!
//! #[derive(EndianSize, MixedEndianSerialize)]
//! // You work with very old system and there are mixed endianness
//! // There will be only one format "le" or "little" in next minor version.
//! struct Request {
//!   #[endian = "le"]
//!   cmd: u16,
//!   #[endian = "little"] // or #[endian = "le"]
//!   value: i64,
//!   #[endian = "big"] // or #[endian = "be"]
//!   timestamp: i128,
//! }
//!
//! let mut buf = [0; Request::BYTES_LEN];
//! let req = Request {
//!   cmd: 0x44,
//!   value: 74,
//!   timestamp: 0xFFFF_FFFF_0000_0000,
//! };
//! // here we see me (mixed endian) just look on struct definition for deatels
//! req.serialize_as_me_bytes(&mut buf);
//!
//! ```
//!
//! ### Why another crate to handle endian?
//! * learn how to create custom derives
//! * make a cleaner API
//!
//! ### There are few other crates that deal with endian:
//! * [byteorder] -  Library for reading/writing numbers in big-endian and little-endian.
//! * [bytes] - Buf and BufMut traits have methods to put and get primitives in wanted endian.
//! * [simple_endian] - Instead of provide functions that converts - create types that store
//! variables in wanted endian.
//! * [struct_deser] - Inspiration for this crate - but in more clean and rusty way.
//!
//!
//! [bincode]:https://crates.io/crates/bincode
//! [byteorder]:https://crates.io/crates/byteorder
//! [bytes]:https://crates.io/crates/bytes
//! [simple_endian]:https://crates.io/crates/simple_endian
//! [struct_deser]:https://crates.io/crates/struct_deser
//! [no_std]:https://rust-embedded.github.io/book/intro/no-std.html

#![no_std]
#[cfg(feature = "endian_serde_derive")]
pub use endian_serde_derive::*;

/// Serialized as little endian bytes.
pub trait LittleEndianSerialize: EndianSize {
    fn serialize_as_le_bytes(&self, bytes: &mut [u8]);
}

/// Serialized as big endian bytes.
pub trait BigEndianSerialize: EndianSize {
    fn serialize_as_be_bytes(&self, bytes: &mut [u8]);
}

/// Serialize using mixed endian bytes.
///
/// # Note
/// If you only use big/little endian consider use [BigEndianSerialize](BigEndianSerialize) / [LittleEndianSerialize](LittleEndianSerialize) traits.
pub trait MixedEndianSerialize: EndianSize {
    fn serialize_as_me_bytes(&self, bytes: &mut [u8]);
}

/// Deserialize from bytes stored as little endian.
pub trait LittleEndianDeserialize: EndianSize {
    fn deserialize_from_le_bytes(bytes: &[u8]) -> Self;
}

/// Deserialize from bytes stored as big endian.
pub trait BigEndianDeserialize: EndianSize {
    fn deserialize_from_be_bytes(bytes: &[u8]) -> Self;
}

/// Deserialize from bytes stored as mixed endian.
///
/// # Note
/// If you only use big/little endian consider use [BigEndianDeserialize](BigEndianDeserialize) / [LittleEndianDeserialize](LittleEndianDeserialize) traits.
pub trait MixedEndianDeserialize: EndianSize {
    fn deserialize_from_me_bytes(bytes: &[u8]) -> Self;
}

/// Represent size of struct when is serialized as bytes.
pub trait EndianSize {
    const BYTES_LEN: usize;
}

macro_rules! impl_serde_for_primitives {
    ($type:ty, $byte_len:expr) => {
        impl EndianSize for $type {
            const BYTES_LEN: usize = $byte_len;
        }

        impl LittleEndianSerialize for $type {
            fn serialize_as_le_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(&(self.to_le_bytes()))
            }
        }

        impl BigEndianSerialize for $type {
            fn serialize_as_be_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(&(self.to_be_bytes()))
            }
        }

        impl LittleEndianDeserialize for $type {
            fn deserialize_from_le_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; $byte_len];
                arr.copy_from_slice(&bytes);
                Self::from_le_bytes(arr)
            }
        }

        impl BigEndianDeserialize for $type {
            fn deserialize_from_be_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; $byte_len];
                arr.copy_from_slice(&bytes);
                Self::from_be_bytes(arr)
            }
        }
    };
}

impl_serde_for_primitives!(u16, 2);
impl_serde_for_primitives!(i16, 2);
impl_serde_for_primitives!(u32, 4);
impl_serde_for_primitives!(i32, 4);
impl_serde_for_primitives!(u64, 8);
impl_serde_for_primitives!(i64, 8);
impl_serde_for_primitives!(u128, 16);
impl_serde_for_primitives!(i128, 16);

macro_rules! impl_serde_for_array {
    ($type:ty, $size:expr) => {
        impl EndianSize for $type {
            const BYTES_LEN: usize = $size;
        }

        impl BigEndianSerialize for $type {
            fn serialize_as_be_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self);
            }
        }

        impl LittleEndianSerialize for $type {
            fn serialize_as_le_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self);
            }
        }

        impl MixedEndianSerialize for $type {
            fn serialize_as_me_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self);
            }
        }

        impl BigEndianDeserialize for $type {
            fn deserialize_from_be_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::BYTES_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }

        impl LittleEndianDeserialize for $type {
            fn deserialize_from_le_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::BYTES_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }

        impl MixedEndianDeserialize for $type {
            fn deserialize_from_me_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; Self::BYTES_LEN];
                arr.copy_from_slice(bytes);
                arr
            }
        }
    };
}

impl_serde_for_array!([u8; 1], 1);
impl_serde_for_array!([u8; 2], 2);
impl_serde_for_array!([u8; 3], 3);
impl_serde_for_array!([u8; 4], 4);
impl_serde_for_array!([u8; 5], 5);
impl_serde_for_array!([u8; 6], 6);
impl_serde_for_array!([u8; 7], 7);
impl_serde_for_array!([u8; 8], 8);
impl_serde_for_array!([u8; 9], 9);
impl_serde_for_array!([u8; 10], 10);
impl_serde_for_array!([u8; 11], 11);
impl_serde_for_array!([u8; 12], 12);
impl_serde_for_array!([u8; 13], 13);
impl_serde_for_array!([u8; 14], 14);
impl_serde_for_array!([u8; 15], 15);
impl_serde_for_array!([u8; 16], 16);
impl_serde_for_array!([u8; 17], 17);
impl_serde_for_array!([u8; 18], 18);
impl_serde_for_array!([u8; 19], 19);
impl_serde_for_array!([u8; 20], 20);
impl_serde_for_array!([u8; 21], 21);
impl_serde_for_array!([u8; 22], 22);
impl_serde_for_array!([u8; 23], 23);
impl_serde_for_array!([u8; 24], 24);
impl_serde_for_array!([u8; 25], 25);
impl_serde_for_array!([u8; 26], 26);
impl_serde_for_array!([u8; 27], 27);
impl_serde_for_array!([u8; 28], 28);
impl_serde_for_array!([u8; 29], 29);
impl_serde_for_array!([u8; 30], 30);
impl_serde_for_array!([u8; 31], 31);
impl_serde_for_array!([u8; 32], 32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_endian_size() {
        #[derive(EndianSize)]
        struct A {};
        assert_eq!(A::BYTES_LEN, 0);

        #[derive(EndianSize)]
        struct B {
            _a: u16,
        }
        assert_eq!(B::BYTES_LEN, 2);

        #[derive(EndianSize)]
        struct C {
            _a: u16,
            _b: u16,
        }
        assert_eq!(C::BYTES_LEN, 2 + 2);
    }

    #[test]
    fn derive_littlendian_serialize() {
        #[derive(EndianSize, LittleEndianSerialize)]
        struct Example {
            a: u16,
        }

        let t = Example { a: 5 };
        let mut b = [0; 2];
        t.serialize_as_le_bytes(&mut b);
    }

    #[test]
    fn derive_bigendian_serialize() {
        #[derive(EndianSize, BigEndianSerialize)]
        struct Example {
            a: u16,
        }

        let t = Example { a: 5 };
        let mut b = [0; 2];
        t.serialize_as_be_bytes(&mut b);
    }

    #[test]
    fn derive_mixed_endian_serialize() {
        #[derive(EndianSize, MixedEndianSerialize, Default)]
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
        t.serialize_as_me_bytes(&mut b);
    }

    fn derive_all_serialize() {
        #[derive(
            Default, EndianSize, LittleEndianSerialize, BigEndianSerialize, MixedEndianSerialize,
        )]
        struct Example {
            #[endian = "be"]
            a: u16,
            b: [u8; 32],
        }

        let t = Example::default();
        let mut b = [0; 2];
        t.serialize_as_me_bytes(&mut b);
        t.serialize_as_be_bytes(&mut b);
        t.serialize_as_le_bytes(&mut b);
    }

    fn derive_all() {
        #[derive(
            Default,
            EndianSize,
            LittleEndianSerialize,
            BigEndianSerialize,
            MixedEndianSerialize,
            LittleEndianDeserialize,
            BigEndianDeserialize,
            MixedEndianDeserialize,
        )]
        struct Example {
            #[endian = "be"]
            a: u16,
        }

        let t = Example::default();
        let mut b = [0; 2];
        t.serialize_as_me_bytes(&mut b);
        t.serialize_as_be_bytes(&mut b);
        t.serialize_as_le_bytes(&mut b);
    }

    /*
    #[test]
    fn derive_parameters() {
        #[derive(LittleEndianSerialize)]
        struct Example<A> {
            a: A,
            #[endian(be)]
            be: u16,
        }

        // jak to sie ma zachować w przypadku kiedy a: nie jest primitivem czyli jest dla nie
        // zaimplemenotwane  EndianSerBytes a kiedy jest to u32. W przypadku u32 takie coś nie
        // 1. w przypadku u32 takie coś nie powinno działać
        // 2. w przypadku A: EndianSerBytes powinno.
    }
    */
}
