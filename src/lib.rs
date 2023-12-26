//! Testing library for [`serde`] [`Serialize`] and [`Deserialize`] implementations.
//!
//! This library provides a [`Serializer`] and [`Deserializer`] to be used in writing unit tests to
//! assert the behavior of manual [`Serialize`] and [`Deserialize`] implementations, respectively.
//! The implementation behavior can be verified by using a sequence of [`Token`]s representing an
//! arbitrary serialized state.
//!
//! # Testing Serialization
//! The [`Serializer`] returns a sequence of [`Token`]s representing the serialization of a value.
//! The returned `Token`s can be checked to be equal to an expected value. Since
//! [`Serialize::serialize()`] returns a `Result<Tokens, Error>`, it is recommended to use the
//! [`claims`] crate to check that the returned value is both `Ok` and equal to the expected
//! sequence of `Token`s.
//!
//! ```
//! use claims::assert_ok_eq;
//! use serde::Serialize;
//! use serde_assert::{
//!     Serializer,
//!     Token,
//! };
//!
//! let serializer = Serializer::builder().build();
//!
//! assert_ok_eq!(true.serialize(&serializer), [Token::Bool(true)]);
//! ```
//!
//! ## Arbitrary Ordering
//! In cases where the ordering of [`Token`]s does not matter, such as when serializing a
//! [`HashSet`], [`Token::Unordered`] can be used to specify that tokens can be in an arbitrary
//! order.
//!
//! ```
//! use claims::assert_ok_eq;
//! use serde::Serialize;
//! use serde_assert::{
//!     Serializer,
//!     Token,
//! };
//! use std::collections::HashSet;
//!
//! let serializer = Serializer::builder().build();
//!
//! let mut set = HashSet::<u32>::new();
//! set.insert(1);
//! set.insert(2);
//! set.insert(3);
//!
//! assert_ok_eq!(
//!     set.serialize(&serializer),
//!     [
//!         Token::Seq { len: Some(3) },
//!         Token::Unordered(&[&[Token::U32(1)], &[Token::U32(2)], &[Token::U32(3)],]),
//!         Token::SeqEnd
//!     ]
//! );
//! ```
//!
//! # Testing Deserialization
//! A [`Deserializer`] is constructed by providing a sequence of [`Token`]s to be deserialized into
//! a value. During testing, the [`claims`] crate can be used to assert that deserialization
//! succeeds and returns the expected value.
//!
//! ```
//! use claims::assert_ok_eq;
//! use serde::Deserialize;
//! use serde_assert::{
//!     Deserializer,
//!     Token,
//! };
//!
//! let mut deserializer = Deserializer::builder([Token::Bool(true)]).build();
//!
//! assert_ok_eq!(bool::deserialize(&mut deserializer), true);
//! ```
//!
//! # Testing Roundtrip
//! To assert that a value remains the same when serialized and then deserialized again, the output
//! of the [`Serializer`] can be used as input to the [`Deserializer`].
//!
//! ```
//! use claims::{
//!     assert_ok,
//!     assert_ok_eq,
//! };
//! use serde::{
//!     Deserialize,
//!     Serialize,
//! };
//! use serde_assert::{
//!     Deserializer,
//!     Serializer,
//! };
//!
//! let value = true;
//!
//! let serializer = Serializer::builder().build();
//! let mut deserializer = Deserializer::builder(assert_ok!(value.serialize(&serializer))).build();
//!
//! assert_ok_eq!(bool::deserialize(&mut deserializer), value);
//! ```
//!
//! [`claims`]: https://docs.rs/claims/
//! [`Deserialize`]: serde::Deserialize
//! [`HashSet`]: std::collections::HashSet
//! [`Serialize`]: serde::Serialize
//! [`Serialize::serialize()`]: serde::Serialize::serialize()

#![no_std]
#![warn(clippy::pedantic)]

extern crate alloc;
#[cfg(any(test, doc))]
extern crate std;

pub mod de;
pub mod ser;
pub mod token;

#[doc(inline)]
pub use de::Deserializer;
#[doc(inline)]
pub use ser::Serializer;
#[doc(inline)]
pub use token::Token;
