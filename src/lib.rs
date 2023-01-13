//! Testing library for [`serde`] [`Serialize`] and [`Deserialize`] implementations.
//!
//! This library provides a [`Serializer`] and [`Deserializer`] to be used in writing unit tests to
//! assert the behavior of manual [`Serialize`] and [`Deserialize`] implementations, respectively.
//! The implementation behavior can be verified by using [`Tokens`] representing an arbitrary
//! serialized state.
//!
//! # Testing Serialization
//! The [`Serializer`] returns [`Tokens`] representing the serialization of a value. The returned
//! `Tokens` can be checked to be equal to an expected value. Since [`Serialize::serialize()`]
//! returns a `Result<Tokens, Error>`, it is recommended to use the [`claims`] crate to check that
//! the returned value is both `Ok` and equal to the expected `Tokens`.
//!
//! ```
//! use claims::assert_ok_eq;
//! use serde::Serialize;
//! use serde_assert::{
//!     Serializer,
//!     Token,
//!     Tokens,
//! };
//!
//! let serializer = Serializer::builder().build();
//!
//! assert_ok_eq!(true.serialize(&serializer), Tokens(vec![Token::Bool(true)]));
//! ```
//!
//! ## Arbitrary Ordering
//! In cases where the ordering of [`Token`]s does not matter, such as when serializing a
//! [`HashSet`], [`Token::Unordered`] can be used to specify that tokens can be in an arbitrary
//! order.
//!
//! ```
//! use claims::assert_ok_eq;
//! use hashbrown::HashSet;
//! use serde::Serialize;
//! use serde_assert::{
//!     Serializer,
//!     Token,
//!     Tokens,
//! };
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
//!     Tokens(vec![
//!         Token::Seq { len: Some(3) },
//!         Token::Unordered(&[&[Token::U32(1)], &[Token::U32(2)], &[Token::U32(3)],]),
//!         Token::SeqEnd
//!     ])
//! );
//! ```
//!
//! # Testing Deserialization
//! A [`Deserializer`] is constructed by providing [`Tokens`] to be deserialized into a value.
//! During testing, the [`claims`] crate can be used to assert that deserialization succeeds and
//! returns the expected value.
//!
//! ```
//! use claims::assert_ok_eq;
//! use serde::Deserialize;
//! use serde_assert::{
//!     Deserializer,
//!     Token,
//!     Tokens,
//! };
//!
//! let mut deserializer = Deserializer::builder()
//!     .tokens(Tokens(vec![Token::Bool(true)]))
//!     .build();
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
//! let mut deserializer = Deserializer::builder()
//!     .tokens(assert_ok!(value.serialize(&serializer)))
//!     .build();
//!
//! assert_ok_eq!(bool::deserialize(&mut deserializer), value);
//! ```
//!
//! [`claims`]: https://docs.rs/claims/
//! [`Deserialize`]: serde::Deserialize
//! [`HashSet`]: hashbrown::HashSet
//! [`Serialize`]: serde::Serialize
//! [`Serialize::serialize()`]: serde::Serialize::serialize()

#![no_std]
#![warn(clippy::pedantic)]

extern crate alloc;

pub mod de;
pub mod ser;

mod token;

#[doc(inline)]
pub use de::Deserializer;
#[doc(inline)]
pub use ser::Serializer;
pub use token::{
    Token,
    Tokens,
};
