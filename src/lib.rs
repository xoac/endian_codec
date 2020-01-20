//! This crate is to help serialize types as bytes and deserialize from bytes with special
//! bytes order. This crate can be used in no_std environment and has no external dependencies.
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
//! There can be also situation when you ~want~ are forced to work with mixed endian in one struct.
//! We aiming to
//! ```rust
//! use endian_serde::{EndianSize, MixedEndianSerialize};
//! // even if you use only derive MixedEndianSerialize you also need used serialized tratis in
//! // scope
//! use endian_serde::{LittleEndianSerialize, BigEndianSerialize}; // for #[endian = "le/be"]
//!
//! #[derive(EndianSize, MixedEndianSerialize)]
//! // You work with very old system and there are mixed endian
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
//! Why another crate to handle endian?
//! In my opinion any that currently exist do it clean. And the second reason I want to learn how
//! to create derive crate.
//!
//! There are few other crates that deal with endian:
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

#![no_std]
#[cfg(feature = "endian_serde_derive")]
pub use endian_serde_derive::*;

/// Represent type that can be serialized as little endian bytes.
pub trait LittleEndianSerialize: EndianSize {
    fn serialize_as_le_bytes(&self, bytes: &mut [u8]);
}

/// Represent type that can be serialized as big endian bytes.
pub trait BigEndianSerialize: EndianSize {
    fn serialize_as_be_bytes(&self, bytes: &mut [u8]);
}

/// Represent type that is serialized using mixed endian bytes.
///
/// # Note
/// If you only use big/little endian consider use [BigEndianSerialize](BigEndianSerialize) / [LittleEndianSerialize](LittleEndianSerialize) traits.
pub trait MixedEndianSerialize: EndianSize {
    fn serialize_as_me_bytes(&self, bytes: &mut [u8]);
}

/// Represent type that can be deserialize from bytes stored as little endian.
pub trait LittleEndianDeserialize: EndianSize {
    fn deserialize_from_le_bytes(bytes: &[u8]) -> Self;
}

/// Represent type that can be deserialize from bytes stored as big endian.
pub trait BigEndianDeserialize: EndianSize {
    fn deserialize_from_be_bytes(bytes: &[u8]) -> Self;
}

/// Represent type that is deserialize from bytes stored as mixed endian.
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

macro_rules! impl_serialize_for_primitives {
    ($type:ty, $byte_len:expr, $le_function:ident, $be_function:ident) => {
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
    };
}

impl_serialize_for_primitives!(u16, 2, put_u16_le, put_u16);
impl_serialize_for_primitives!(i16, 2, put_i16_le, put_i16);
impl_serialize_for_primitives!(u32, 4, put_u32_le, put_u32);
impl_serialize_for_primitives!(i32, 4, put_i32_le, put_i32);
impl_serialize_for_primitives!(u64, 8, put_u64_le, put_u64);
impl_serialize_for_primitives!(i64, 8, put_i64_le, put_i64);
impl_serialize_for_primitives!(u128, 16, put_u128_le, put_u128);
impl_serialize_for_primitives!(i128, 16, put_i128_le, put_i128);

macro_rules! impl_deserialize_for_primitives {
    ($type:ty, $byte_len:expr, $le_function:ident, $be_function:ident) => {
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

impl_deserialize_for_primitives!(u16, 2, put_u16_le, put_u16);
impl_deserialize_for_primitives!(i16, 2, put_i16_le, put_i16);
impl_deserialize_for_primitives!(u32, 4, put_u32_le, put_u32);
impl_deserialize_for_primitives!(i32, 4, put_i32_le, put_i32);
impl_deserialize_for_primitives!(u64, 8, put_u64_le, put_u64);
impl_deserialize_for_primitives!(i64, 8, put_i64_le, put_i64);
impl_deserialize_for_primitives!(u128, 16, put_u128_le, put_u128);
impl_deserialize_for_primitives!(i128, 16, put_i128_le, put_i128);

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
