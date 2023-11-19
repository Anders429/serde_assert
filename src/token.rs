//! Tokens representing a serialized object.
//!
//! This module provides a [`Token`] type for representing a serialized value, as well as a
//! [`Tokens`] type containing a set of `Token`s. `Tokens` can be compared for equality, which is
//! useful for asserting whether serialized `Tokens` are as expected.

use alloc::{
    slice,
    string::String,
    vec::Vec,
};
use core::{
    fmt,
    fmt::{
        Debug,
        Display,
    },
    iter,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};
use hashbrown::HashSet;
use serde::de::Unexpected;

/// A single serialized value.
///
/// A `Token` is a single serialization output produced by the [`Serializer`]. The one exception to
/// this is the [`Unordered`] variant, which contains multiple sets of tokens that can be in any
/// order. This is never produced by the `Serializer`, and is for use when comparing equality of
/// [`Tokens`].
///
/// Normally, `Token`s are used within the [`Tokens`] struct to either compare against the output
/// of a [`Serializer`] or to be used as input to a [`Deserializer`].
///
/// [`Deserializer`]: crate::Deserializer
/// [`Serializer`]: crate::Serializer
/// [`Unordered`]: Token::Unordered
#[derive(Clone, Debug)]
pub enum Token {
    /// A [`bool`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(true.serialize(&serializer), Tokens(vec![Token::Bool(true)]));
    /// ```
    Bool(bool),

    /// An [`i8`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i8.serialize(&serializer), Tokens(vec![Token::I8(42)]));
    /// ```
    I8(i8),

    /// An [`i16`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i16.serialize(&serializer), Tokens(vec![Token::I16(42)]));
    /// ```
    I16(i16),

    /// An [`i32`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i32.serialize(&serializer), Tokens(vec![Token::I32(42)]));
    /// ```
    I32(i32),

    /// An [`i64`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i64.serialize(&serializer), Tokens(vec![Token::I64(42)]));
    /// ```
    I64(i64),

    #[cfg(has_i128)]
    /// An [`i128`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i128.serialize(&serializer), Tokens(vec![Token::I128(42)]));
    /// ```
    I128(i128),

    /// A [`u8`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u8.serialize(&serializer), Tokens(vec![Token::U8(42)]));
    /// ```
    U8(u8),

    /// A [`u16`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u16.serialize(&serializer), Tokens(vec![Token::U16(42)]));
    /// ```
    U16(u16),

    /// A [`u32`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u32.serialize(&serializer), Tokens(vec![Token::U32(42)]));
    /// ```
    U32(u32),

    /// A [`u64`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u64.serialize(&serializer), Tokens(vec![Token::U64(42)]));
    /// ```
    U64(u64),

    #[cfg(has_i128)]
    /// A [`u128`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u128.serialize(&serializer), Tokens(vec![Token::U128(42)]));
    /// ```
    U128(u128),

    /// A [`f32`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     42.0f32.serialize(&serializer),
    ///     Tokens(vec![Token::F32(42.0)])
    /// );
    /// ```
    F32(f32),

    /// A [`f64`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     42.0f64.serialize(&serializer),
    ///     Tokens(vec![Token::F64(42.0)])
    /// );
    /// ```
    F64(f64),

    /// A [`char`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!('a'.serialize(&serializer), Tokens(vec![Token::Char('a')]));
    /// ```
    Char(char),

    /// A string.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     "foo".serialize(&serializer),
    ///     Tokens(vec![Token::Str("foo".to_owned())])
    /// );
    /// ```
    Str(String),

    /// Bytes.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_bytes::Bytes;
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Bytes::new(b"foo").serialize(&serializer),
    ///     Tokens(vec![Token::Bytes(b"foo".to_vec())])
    /// );
    /// ```
    Bytes(Vec<u8>),

    /// An [`Option::None`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Option::<()>::None.serialize(&serializer),
    ///     Tokens(vec![Token::None])
    /// );
    /// ```
    None,

    /// An [`Option::Some`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Some(()).serialize(&serializer),
    ///     Tokens(vec![Token::Some, Token::Unit])
    /// );
    /// ```
    Some,

    /// A unit.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(().serialize(&serializer), Tokens(vec![Token::Unit]));
    /// ```
    Unit,

    /// A unit struct.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct UnitStruct;
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     UnitStruct.serialize(&serializer),
    ///     Tokens(vec![Token::UnitStruct { name: "UnitStruct" }])
    /// );
    /// ```
    UnitStruct { name: &'static str },

    /// A unit variant on an `enum`.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// enum Enum {
    ///     Unit,
    /// }
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Enum::Unit.serialize(&serializer),
    ///     Tokens(vec![Token::UnitVariant {
    ///         name: "Enum",
    ///         variant_index: 0,
    ///         variant: "Unit"
    ///     }])
    /// );
    /// ```
    UnitVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    },

    /// A newtype struct.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct NewtypeStruct(u32);
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     NewtypeStruct(42).serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::NewtypeStruct {
    ///             name: "NewtypeStruct"
    ///         },
    ///         Token::U32(42)
    ///     ])
    /// );
    /// ```
    NewtypeStruct { name: &'static str },

    /// A newtype variant on an `enum`.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// enum Enum {
    ///     Newtype(u32),
    /// }
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Enum::Newtype(42).serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::NewtypeVariant {
    ///             name: "Enum",
    ///             variant_index: 0,
    ///             variant: "Newtype"
    ///         },
    ///         Token::U32(42)
    ///     ])
    /// );
    /// ```
    NewtypeVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    },

    /// A sequence.
    ///
    /// Must be followed by a [`SeqEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     vec![1u32, 2u32, 3u32].serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::Seq { len: Some(3) },
    ///         Token::U32(1),
    ///         Token::U32(2),
    ///         Token::U32(3),
    ///         Token::SeqEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`SeqEnd`]: Token::SeqEnd
    Seq { len: Option<usize> },

    /// The end of a sequence.
    ///
    /// This token must follow a [`Seq`] token.
    ///
    /// [`Seq`]: Token::Seq
    SeqEnd,

    /// A tuple.
    ///
    /// Must be followed by a [`TupleEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     (42u32, true).serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::Tuple { len: 2 },
    ///         Token::U32(42),
    ///         Token::Bool(true),
    ///         Token::TupleEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`TupleEnd`]: Token::TupleEnd
    Tuple { len: usize },

    /// The end of a tuple.
    ///
    /// This token must follow a [`Tuple`] token.
    ///
    /// [`Tuple`]: Token::Tuple
    TupleEnd,

    /// A tuple struct.
    ///
    /// Must be followed by a [`TupleStructEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct TupleStruct(u32, bool);
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     TupleStruct(42u32, true).serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::TupleStruct {
    ///             name: "TupleStruct",
    ///             len: 2
    ///         },
    ///         Token::U32(42),
    ///         Token::Bool(true),
    ///         Token::TupleStructEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`TupleStructEnd`]: Token::TupleStructEnd
    TupleStruct { name: &'static str, len: usize },

    /// The end of a tuple struct.
    ///
    /// This token must follow a [`TupleStruct`] token.
    ///
    /// [`TupleStruct`]: Token::TupleStruct
    TupleStructEnd,

    /// A tuple variant on an `enum`.
    ///
    /// Must be followed by a [`TupleVariantEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// enum Enum {
    ///     Tuple(u32, bool),
    /// }
    /// struct TupleStruct(u32, bool);
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Enum::Tuple(42u32, true).serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::TupleVariant {
    ///             name: "Enum",
    ///             variant_index: 0,
    ///             variant: "Tuple",
    ///             len: 2
    ///         },
    ///         Token::U32(42),
    ///         Token::Bool(true),
    ///         Token::TupleVariantEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`TupleVariantEnd`]: Token::TupleVariantEnd
    TupleVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    },

    /// The end of a tuple variant.
    ///
    /// This token must follow a [`TupleVariant`] token.
    ///
    /// [`TupleVariant`]: Token::TupleVariant
    TupleVariantEnd,

    /// A map.
    ///
    /// Must be followed by a [`MapEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use hashbrown::HashMap;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// let mut map = HashMap::new();
    /// map.insert("foo", 42u32);
    ///
    /// assert_ok_eq!(
    ///     map.serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::Map { len: Some(1) },
    ///         Token::Str("foo".to_owned()),
    ///         Token::U32(42),
    ///         Token::MapEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`MapEnd`]: Token::MapEnd
    Map { len: Option<usize> },

    /// The end of a map.
    ///
    /// This token must follow a [`Map`] token.
    ///
    /// [`Map`]: Token::Map
    MapEnd,

    /// A field within a [`Struct`].
    ///
    /// [`Struct`]: Token::Struct
    Field(&'static str),

    /// A field within a [`Struct`], skipped during serialization.
    ///
    /// This token is emitted when the [`SerializeStruct::skip_field()`] method is called during
    /// serialization.
    ///
    /// [`SerializeStruct::skip_field()`]: serde::ser::SerializeStruct::skip_field()
    /// [`Struct`]: Token::Struct
    SkippedField(&'static str),

    /// A struct.
    ///
    /// Must be followed by a [`StructEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Struct {
    ///     foo: u32,
    ///     bar: bool,
    /// }
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Struct {
    ///         foo: 42u32,
    ///         bar: true
    ///     }
    ///     .serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::Struct {
    ///             name: "Struct",
    ///             len: 2
    ///         },
    ///         Token::Field("foo"),
    ///         Token::U32(42),
    ///         Token::Field("bar"),
    ///         Token::Bool(true),
    ///         Token::StructEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`StructEnd`]: Token::StructEnd
    Struct { name: &'static str, len: usize },

    /// The end of a struct.
    ///
    /// This token must follow a [`Struct`] token.
    ///
    /// [`Struct`]: Token::Struct
    StructEnd,

    /// A struct variant on an `enum`.
    ///
    /// Must be followed by a [`StructVariantEnd`] token.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    /// use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// enum Enum {
    ///     Struct { foo: u32, bar: bool },
    /// }
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Enum::Struct {
    ///         foo: 42u32,
    ///         bar: true
    ///     }
    ///     .serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::StructVariant {
    ///             name: "Enum",
    ///             variant_index: 0,
    ///             variant: "Struct",
    ///             len: 2
    ///         },
    ///         Token::Field("foo"),
    ///         Token::U32(42),
    ///         Token::Field("bar"),
    ///         Token::Bool(true),
    ///         Token::StructVariantEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`StructVariantEnd`]: Token::StructVariantEnd
    StructVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    },

    /// The end of a struct variant.
    ///
    /// This token must follow a [`StructVariant`] token.
    ///
    /// [`StructVariant`]: Token::StructVariant
    StructVariantEnd,

    /// Unordered sets of tokens.
    ///
    /// This token is primarily used for evaluating output from a [`Serializer`] for containers or
    /// other types whose internal ordering is not defined (such as a [`HashSet`]).
    ///
    /// This is a set of groups of tokens, where the groups may appear in any order when comparing
    /// equality of [`Tokens`]. In other words, the outer slice is unordered, while the inner
    /// slices are all ordered.
    ///
    /// Note that comparing equality of nested `Unordered` tokens is not currently supported, and
    /// may give bogus results.
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use hashbrown::HashMap;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    ///     Tokens,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// let mut map = HashMap::<char, u32>::new();
    /// map.insert('a', 1);
    /// map.insert('b', 2);
    /// map.insert('c', 3);
    ///
    /// assert_ok_eq!(
    ///     map.serialize(&serializer),
    ///     Tokens(vec![
    ///         Token::Map { len: Some(3) },
    ///         Token::Unordered(&[
    ///             &[Token::Char('a'), Token::U32(1)],
    ///             &[Token::Char('b'), Token::U32(2)],
    ///             &[Token::Char('c'), Token::U32(3)]
    ///         ]),
    ///         Token::MapEnd
    ///     ])
    /// );
    /// ```
    ///
    /// [`HashSet`]: hashbrown::HashSet
    /// [`Serializer`]: crate::Serializer
    Unordered(&'static [&'static [Token]]),
}

impl PartialEq for Token {
    #[allow(clippy::too_many_lines)] // The large amount of lines comes from the large amount of variants.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Bool(a), Token::Bool(b)) => a == b,
            (Token::I8(a), Token::I8(b)) => a == b,
            (Token::I16(a), Token::I16(b)) => a == b,
            (Token::I32(a), Token::I32(b)) => a == b,
            (Token::I64(a), Token::I64(b)) => a == b,
            #[cfg(has_i128)]
            (Token::I128(a), Token::I128(b)) => a == b,
            (Token::U8(a), Token::U8(b)) => a == b,
            (Token::U16(a), Token::U16(b)) => a == b,
            (Token::U32(a), Token::U32(b)) => a == b,
            (Token::U64(a), Token::U64(b)) => a == b,
            #[cfg(has_i128)]
            (Token::U128(a), Token::U128(b)) => a == b,
            (Token::F32(a), Token::F32(b)) => a == b,
            (Token::F64(a), Token::F64(b)) => a == b,
            (Token::Char(a), Token::Char(b)) => a == b,
            (Token::Str(a), Token::Str(b)) => a == b,
            (Token::Bytes(a), Token::Bytes(b)) => a == b,
            (Token::None, Token::None)
            | (Token::Some, Token::Some)
            | (Token::Unit, Token::Unit)
            | (Token::SeqEnd, Token::SeqEnd)
            | (Token::TupleEnd, Token::TupleEnd)
            | (Token::TupleStructEnd, Token::TupleStructEnd)
            | (Token::TupleVariantEnd, Token::TupleVariantEnd)
            | (Token::MapEnd, Token::MapEnd)
            | (Token::StructEnd, Token::StructEnd)
            | (Token::StructVariantEnd, Token::StructVariantEnd) => true,
            (Token::UnitStruct { name: name_a }, Token::UnitStruct { name: name_b })
            | (Token::NewtypeStruct { name: name_a }, Token::NewtypeStruct { name: name_b }) => {
                name_a == name_b
            }
            (
                Token::UnitVariant {
                    name: name_a,
                    variant_index: variant_index_a,
                    variant: variant_a,
                },
                Token::UnitVariant {
                    name: name_b,
                    variant_index: variant_index_b,
                    variant: variant_b,
                },
            )
            | (
                Token::NewtypeVariant {
                    name: name_a,
                    variant_index: variant_index_a,
                    variant: variant_a,
                },
                Token::NewtypeVariant {
                    name: name_b,
                    variant_index: variant_index_b,
                    variant: variant_b,
                },
            ) => name_a == name_b && variant_index_a == variant_index_b && variant_a == variant_b,
            (Token::Seq { len: len_a }, Token::Seq { len: len_b })
            | (Token::Map { len: len_a }, Token::Map { len: len_b }) => len_a == len_b,
            (Token::Tuple { len: len_a }, Token::Tuple { len: len_b }) => len_a == len_b,
            (
                Token::TupleStruct {
                    name: name_a,
                    len: len_a,
                },
                Token::TupleStruct {
                    name: name_b,
                    len: len_b,
                },
            )
            | (
                Token::Struct {
                    name: name_a,
                    len: len_a,
                },
                Token::Struct {
                    name: name_b,
                    len: len_b,
                },
            ) => name_a == name_b && len_a == len_b,
            (
                Token::TupleVariant {
                    name: name_a,
                    variant_index: variant_index_a,
                    variant: variant_a,
                    len: len_a,
                },
                Token::TupleVariant {
                    name: name_b,
                    variant_index: variant_index_b,
                    variant: variant_b,
                    len: len_b,
                },
            )
            | (
                Token::StructVariant {
                    name: name_a,
                    variant_index: variant_index_a,
                    variant: variant_a,
                    len: len_a,
                },
                Token::StructVariant {
                    name: name_b,
                    variant_index: variant_index_b,
                    variant: variant_b,
                    len: len_b,
                },
            ) => {
                name_a == name_b
                    && variant_index_a == variant_index_b
                    && variant_a == variant_b
                    && len_a == len_b
            }
            (Token::Field(a), Token::Field(b))
            | (Token::SkippedField(a), Token::SkippedField(b)) => a == b,
            (Token::Unordered(tokens_a), Token::Unordered(tokens_b)) => {
                if tokens_a.len() != tokens_b.len() {
                    return false;
                }

                let mut consumed = HashSet::new();
                'outer: for tokens in *tokens_a {
                    for (i, other_tokens) in tokens_b
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !consumed.contains(i))
                    {
                        if tokens == other_tokens {
                            consumed.insert(i);
                            continue 'outer;
                        }
                    }
                    return false;
                }
                true
            }
            _ => false,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'a> From<&'a Token> for Unexpected<'a> {
    fn from(token: &'a Token) -> Self {
        match token {
            Token::Bool(v) => Unexpected::Bool(*v),
            Token::I8(v) => Unexpected::Signed((*v).into()),
            Token::I16(v) => Unexpected::Signed((*v).into()),
            Token::I32(v) => Unexpected::Signed((*v).into()),
            Token::I64(v) => Unexpected::Signed(*v),
            #[cfg(has_i128)]
            Token::I128(..) => Unexpected::Other("i128"),
            Token::U8(v) => Unexpected::Unsigned((*v).into()),
            Token::U16(v) => Unexpected::Unsigned((*v).into()),
            Token::U32(v) => Unexpected::Unsigned((*v).into()),
            Token::U64(v) => Unexpected::Unsigned(*v),
            #[cfg(has_i128)]
            Token::U128(..) => Unexpected::Other("u128"),
            Token::F32(v) => Unexpected::Float((*v).into()),
            Token::F64(v) => Unexpected::Float(*v),
            Token::Char(v) => Unexpected::Char(*v),
            Token::Str(v) => Unexpected::Str(v),
            Token::Bytes(v) => Unexpected::Bytes(v),
            Token::Some | Token::None => Unexpected::Option,
            Token::Unit | Token::UnitStruct { .. } => Unexpected::Unit,
            Token::UnitVariant { .. } => Unexpected::UnitVariant,
            Token::NewtypeStruct { .. } => Unexpected::NewtypeStruct,
            Token::NewtypeVariant { .. } => Unexpected::NewtypeVariant,
            Token::Seq { .. } | Token::Tuple { .. } => Unexpected::Seq,
            Token::SeqEnd => Unexpected::Other("SeqEnd"),
            Token::TupleEnd => Unexpected::Other("TupleEnd"),
            Token::TupleStruct { .. } => Unexpected::Other("TupleStruct"),
            Token::TupleStructEnd => Unexpected::Other("TupleStructEnd"),
            Token::TupleVariant { .. } => Unexpected::TupleVariant,
            Token::TupleVariantEnd => Unexpected::Other("TupleVariantEnd"),
            Token::Map { .. } => Unexpected::Map,
            Token::MapEnd => Unexpected::Other("MapEnd"),
            Token::Field(..) => Unexpected::Other("Field"),
            Token::SkippedField(..) => Unexpected::Other("SkippedField"),
            Token::Struct { .. } => Unexpected::Other("Struct"),
            Token::StructEnd => Unexpected::Other("StructEnd"),
            Token::StructVariant { .. } => Unexpected::StructVariant,
            Token::StructVariantEnd => Unexpected::Other("StructVariantEnd"),
            Token::Unordered(..) => Unexpected::Other("unordered tokens"),
        }
    }
}

/// An ordered set of [`Token`]s.
///
/// This is simply a wrapper around a [`Vec<Token>`], providing a custom [`PartialEq`]
/// implementation for comparing with tokens output from a [`Serializer`].
///
/// # Examples
///
/// `Tokens` are output from a [`Serializer`] and are used as input to a [`Deserializer`].
///
/// ## Serialization
///
/// ``` rust
/// use claims::assert_ok_eq;
/// use serde::Serialize;
/// use serde_assert::{
///     Serializer,
///     Token,
///     Tokens,
/// };
///
/// let serializer = Serializer::builder().build();
///
/// assert_ok_eq!(true.serialize(&serializer), Tokens(vec![Token::Bool(true)]));
/// ```
///
/// ## Deserialization
///
/// ``` rust
/// use claims::assert_ok_eq;
/// use serde::Deserialize;
/// use serde_assert::{
///     Deserializer,
///     Token,
///     Tokens,
/// };
///
/// let mut deserializer = Deserializer::builder()
///     .tokens(Tokens(vec![Token::Bool(true)]))
///     .build();
///
/// assert_ok_eq!(bool::deserialize(&mut deserializer), true);
/// ```
///
/// [`Deserializer`]: crate::Deserializer
/// [`Serializer`]: crate::Serializer
#[derive(Clone, Debug)]
pub struct Tokens(pub Vec<Token>);

fn consume_unordered<'a, I>(unordered_tokens: &[&[Token]], mut tokens_iter: I) -> bool
where
    I: Iterator<Item = &'a Token>,
{
    // TODO: Handle nested Unordered tokens.

    /// A current state in the search.
    #[derive(Debug)]
    struct State {
        set_index: usize,
        token_index: usize,
        to_visit: Vec<usize>,
    }

    let mut current = (0..unordered_tokens.len())
        .map(|i| State {
            set_index: i,
            token_index: 0,
            to_visit: (0..unordered_tokens.len()).filter(|&j| i != j).collect(),
        })
        .collect::<Vec<_>>();

    loop {
        if current.is_empty() {
            return false;
        }

        if let Some(token) = tokens_iter.next() {
            let mut new_current = Vec::new();
            for mut state in current {
                if &unordered_tokens[state.set_index][state.token_index] == token {
                    state.token_index += 1;
                    if state.token_index == unordered_tokens[state.set_index].len() {
                        // End condition.
                        if state.to_visit.is_empty() {
                            return true;
                        }
                        for &new_index in &state.to_visit {
                            new_current.push(State {
                                set_index: new_index,
                                token_index: 0,
                                to_visit: state
                                    .to_visit
                                    .iter()
                                    .copied()
                                    .filter(|&i| i != new_index)
                                    .collect(),
                            });
                        }
                    } else {
                        new_current.push(state);
                    }
                }
            }
            current = new_current;
        } else {
            return false;
        };
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        let mut self_iter = self.0.iter();
        let mut other_iter = other.0.iter();

        loop {
            // Obtain next tokens, or return if no tokens are available.
            let self_token;
            loop {
                if let Some(token) = self_iter.next() {
                    if let Token::Unordered(tokens) = token {
                        if tokens.iter().filter(|s| !s.is_empty()).count() == 0 {
                            continue;
                        }
                    }
                    self_token = token;
                    break;
                }
                return other_iter.next().is_none();
            }

            let other_token;
            loop {
                if let Some(token) = other_iter.next() {
                    if let Token::Unordered(tokens) = token {
                        if tokens.iter().filter(|s| !s.is_empty()).count() == 0 {
                            continue;
                        }
                    }
                    other_token = token;
                    break;
                }
                return false;
            }

            match (self_token, other_token) {
                (Token::Unordered(_), Token::Unordered(_)) => {
                    if self_token != other_token {
                        return false;
                    }
                }
                (Token::Unordered(tokens), _) => {
                    if !consume_unordered(tokens, iter::once(other_token).chain(&mut other_iter)) {
                        return false;
                    }
                }
                (_, Token::Unordered(tokens)) => {
                    if !consume_unordered(tokens, iter::once(self_token).chain(&mut self_iter)) {
                        return false;
                    }
                }
                _ => {
                    if self_token != other_token {
                        return false;
                    }
                }
            }
        }
    }
}

/// An iterator over tokens.
///
/// This iterator owns the tokens, iterating over references to them.
pub(crate) struct Iter<'a> {
    /// A pointer to the entire buffer that is owned by this struct.
    ///
    /// Immutable references to the `Token`s in this buffer can exist within the lifetime `'a`.
    buf: NonNull<Token>,
    /// A pointer to the current position in iteration.
    ptr: *const Token,
    /// A pointer to the end of the allocated buffer.
    end: *const Token,
    /// The capacity of the underlying allocation.
    ///
    /// This is only used for deallocating when the struct is dropped.
    cap: usize,

    /// The lifetime of the underlying `Token`s.
    ///
    /// `Token`s can be borrowed for up to the lifetime `'a`, allowing for zero-copy
    /// deserialization.
    lifetime: PhantomData<&'a ()>,
}

impl Iter<'_> {
    /// Creates a new `Iter` from a list of `Tokens`.
    ///
    /// Takes ownership of the `Tokens` and its underlying buffer.
    pub(crate) fn new(tokens: Tokens) -> Self {
        let mut tokens = ManuallyDrop::new(tokens);

        Self {
            // SAFETY: The pointer used by the `Vec` in `Tokens` is guaranteed to not be null.
            buf: unsafe { NonNull::new_unchecked(tokens.0.as_mut_ptr()) },
            ptr: tokens.0.as_ptr(),
            // SAFETY: The resulting pointer is one byte past the end of the allocated object.
            end: unsafe { tokens.0.as_ptr().add(tokens.0.len()) },
            cap: tokens.0.capacity(),

            lifetime: PhantomData,
        }
    }

    /// Returns the remaining `Token`s as a slice.
    fn as_slice(&self) -> &[Token] {
        // SAFETY: `self.ptr` is guaranteed to be less than `self.end`, and therefore a valid
        // pointer within the allocated object.
        unsafe {
            slice::from_raw_parts(
                self.ptr,
                #[allow(clippy::cast_sign_loss)]
                {
                    self.end.offset_from(self.ptr) as usize
                },
            )
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            let current = self.ptr;
            // SAFETY: Since `self.ptr` is not equal to `self.end`, it must be before it, and
            // therefore incrementing by 1 will also result in a valid pointer within the allocated
            // object, or 1 byte past the end if the iteration has completed.
            self.ptr = unsafe { self.ptr.add(1) };
            // SAFETY: The pointed-at object is guaranteed to be a valid `Token` that will live for
            // the lifetime `'a`.
            Some(unsafe { &*current })
        }
    }
}

impl Debug for Iter<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Iter")
            .field(&self.as_slice())
            .finish()
    }
}

impl Drop for Iter<'_> {
    fn drop(&mut self) {
        // SAFETY: The raw parts stored in this struct are guaranteed to correspond to the valid
        // parts of a `Vec`, since the parts were obtained directly from a `Vec` originally.
        unsafe {
            Vec::from_raw_parts(
                self.buf.as_ptr(),
                #[allow(clippy::cast_sign_loss)]
                {
                    self.end.offset_from(self.buf.as_ptr()) as usize
                },
                self.cap,
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Iter,
        Token,
        Tokens,
    };
    use alloc::{
        borrow::ToOwned,
        format,
        vec,
        vec::Vec,
    };
    use claims::{
        assert_none,
        assert_some,
        assert_some_eq,
    };
    use serde::de::Unexpected;

    #[test]
    fn token_bool_eq() {
        assert_eq!(Token::Bool(true), Token::Bool(true));
    }

    #[test]
    fn token_bool_ne() {
        assert_ne!(Token::Bool(true), Token::Bool(false));
    }

    #[test]
    fn token_i8_eq() {
        assert_eq!(Token::I8(42), Token::I8(42));
    }

    #[test]
    fn token_i8_ne() {
        assert_ne!(Token::I8(42), Token::I8(43));
    }

    #[test]
    fn token_i16_eq() {
        assert_eq!(Token::I16(42), Token::I16(42));
    }

    #[test]
    fn token_i16_ne() {
        assert_ne!(Token::I16(42), Token::I16(43));
    }

    #[test]
    fn token_i32_eq() {
        assert_eq!(Token::I32(42), Token::I32(42));
    }

    #[test]
    fn token_i32_ne() {
        assert_ne!(Token::I32(42), Token::I32(43));
    }

    #[test]
    fn token_i64_eq() {
        assert_eq!(Token::I64(42), Token::I64(42));
    }

    #[test]
    fn token_i64_ne() {
        assert_ne!(Token::I64(42), Token::I64(43));
    }

    #[cfg(has_i128)]
    #[test]
    fn token_i128_eq() {
        assert_eq!(Token::I128(42), Token::I128(42));
    }

    #[cfg(has_i128)]
    #[test]
    fn token_i128_ne() {
        assert_ne!(Token::I128(42), Token::I128(43));
    }

    #[test]
    fn token_u8_eq() {
        assert_eq!(Token::U8(42), Token::U8(42));
    }

    #[test]
    fn token_u8_ne() {
        assert_ne!(Token::U8(42), Token::U8(43));
    }

    #[test]
    fn token_u16_eq() {
        assert_eq!(Token::U16(42), Token::U16(42));
    }

    #[test]
    fn token_u16_ne() {
        assert_ne!(Token::U16(42), Token::U16(43));
    }

    #[test]
    fn token_u32_eq() {
        assert_eq!(Token::U32(42), Token::U32(42));
    }

    #[test]
    fn token_u32_ne() {
        assert_ne!(Token::U32(42), Token::U32(43));
    }

    #[test]
    fn token_u64_eq() {
        assert_eq!(Token::U64(42), Token::U64(42));
    }

    #[test]
    fn token_u64_ne() {
        assert_ne!(Token::U64(42), Token::U64(43));
    }

    #[cfg(has_i128)]
    #[test]
    fn token_u128_eq() {
        assert_eq!(Token::U128(42), Token::U128(42));
    }

    #[cfg(has_i128)]
    #[test]
    fn token_u128_ne() {
        assert_ne!(Token::U128(42), Token::U128(43));
    }

    #[test]
    fn token_f32_eq() {
        assert_eq!(Token::F32(42.), Token::F32(42.));
    }

    #[test]
    fn token_f32_ne() {
        assert_ne!(Token::F32(42.), Token::F32(43.));
    }

    #[test]
    fn token_f64_eq() {
        assert_eq!(Token::F64(42.), Token::F64(42.));
    }

    #[test]
    fn token_f64_ne() {
        assert_ne!(Token::F64(42.), Token::F64(43.));
    }

    #[test]
    fn token_char_eq() {
        assert_eq!(Token::Char('a'), Token::Char('a'));
    }

    #[test]
    fn token_char_ne() {
        assert_ne!(Token::Char('a'), Token::Char('b'));
    }

    #[test]
    fn token_str_eq() {
        assert_eq!(Token::Str("a".to_owned()), Token::Str("a".to_owned()));
    }

    #[test]
    fn token_str_ne() {
        assert_ne!(Token::Str("a".to_owned()), Token::Str("b".to_owned()));
    }

    #[test]
    fn token_bytes_eq() {
        assert_eq!(Token::Bytes(b"a".to_vec()), Token::Bytes(b"a".to_vec()));
    }

    #[test]
    fn token_bytes_ne() {
        assert_ne!(Token::Bytes(b"a".to_vec()), Token::Bytes(b"b".to_vec()));
    }

    #[test]
    fn token_none_eq() {
        assert_eq!(Token::None, Token::None);
    }

    #[test]
    fn token_some_eq() {
        assert_eq!(Token::Some, Token::Some);
    }

    #[test]
    fn token_unit_eq() {
        assert_eq!(Token::Unit, Token::Unit);
    }

    #[test]
    fn token_unit_struct_eq() {
        assert_eq!(
            Token::UnitStruct { name: "a" },
            Token::UnitStruct { name: "a" }
        );
    }

    #[test]
    fn token_unit_struct_ne() {
        assert_ne!(
            Token::UnitStruct { name: "a" },
            Token::UnitStruct { name: "b" }
        );
    }

    #[test]
    fn token_unit_variant_eq() {
        assert_eq!(
            Token::UnitVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::UnitVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            }
        );
    }

    #[test]
    fn token_unit_variant_ne_name() {
        assert_ne!(
            Token::UnitVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::UnitVariant {
                name: "b",
                variant_index: 1,
                variant: "foo"
            }
        );
    }

    #[test]
    fn token_unit_variant_ne_variant_index() {
        assert_ne!(
            Token::UnitVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::UnitVariant {
                name: "a",
                variant_index: 2,
                variant: "foo"
            }
        );
    }

    #[test]
    fn token_unit_variant_ne_variant() {
        assert_ne!(
            Token::UnitVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::UnitVariant {
                name: "a",
                variant_index: 1,
                variant: "bar"
            }
        );
    }

    #[test]
    fn token_newtype_struct_eq() {
        assert_eq!(
            Token::NewtypeStruct { name: "a" },
            Token::NewtypeStruct { name: "a" }
        );
    }

    #[test]
    fn token_newtype_struct_ne() {
        assert_ne!(
            Token::NewtypeStruct { name: "a" },
            Token::NewtypeStruct { name: "b" }
        );
    }

    #[test]
    fn token_newtype_variant_eq() {
        assert_eq!(
            Token::NewtypeVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::NewtypeVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            }
        );
    }

    #[test]
    fn token_newtype_variant_ne_name() {
        assert_ne!(
            Token::NewtypeVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::NewtypeVariant {
                name: "b",
                variant_index: 1,
                variant: "foo"
            }
        );
    }

    #[test]
    fn token_newtype_variant_ne_variant_index() {
        assert_ne!(
            Token::NewtypeVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::NewtypeVariant {
                name: "a",
                variant_index: 2,
                variant: "foo"
            }
        );
    }

    #[test]
    fn token_newtype_variant_ne_variant() {
        assert_ne!(
            Token::NewtypeVariant {
                name: "a",
                variant_index: 1,
                variant: "foo"
            },
            Token::NewtypeVariant {
                name: "a",
                variant_index: 1,
                variant: "bar"
            }
        );
    }

    #[test]
    fn token_seq_eq_some() {
        assert_eq!(Token::Seq { len: Some(42) }, Token::Seq { len: Some(42) });
    }

    #[test]
    fn token_seq_eq_none() {
        assert_eq!(Token::Seq { len: None }, Token::Seq { len: None });
    }

    #[test]
    fn token_seq_ne_some() {
        assert_ne!(Token::Seq { len: Some(42) }, Token::Seq { len: Some(43) });
    }

    #[test]
    fn token_seq_ne_some_none() {
        assert_ne!(Token::Seq { len: Some(42) }, Token::Seq { len: None });
    }

    #[test]
    fn token_seq_end_eq() {
        assert_eq!(Token::SeqEnd, Token::SeqEnd);
    }

    #[test]
    fn token_tuple_eq() {
        assert_eq!(Token::Tuple { len: 42 }, Token::Tuple { len: 42 });
    }

    #[test]
    fn token_tuple_ne() {
        assert_ne!(Token::Tuple { len: 42 }, Token::Tuple { len: 43 });
    }

    #[test]
    fn token_tuple_end_eq() {
        assert_eq!(Token::TupleEnd, Token::TupleEnd);
    }

    #[test]
    fn token_tuple_struct_eq() {
        assert_eq!(
            Token::TupleStruct { name: "a", len: 42 },
            Token::TupleStruct { name: "a", len: 42 }
        );
    }

    #[test]
    fn token_tuple_struct_ne_name() {
        assert_ne!(
            Token::TupleStruct { name: "a", len: 42 },
            Token::TupleStruct { name: "b", len: 42 }
        );
    }

    #[test]
    fn token_tuple_struct_ne_len() {
        assert_ne!(
            Token::TupleStruct { name: "a", len: 42 },
            Token::TupleStruct { name: "a", len: 43 }
        );
    }

    #[test]
    fn token_tuple_struct_end_eq() {
        assert_eq!(Token::TupleStructEnd, Token::TupleStructEnd);
    }

    #[test]
    fn token_tuple_variant_eq() {
        assert_eq!(
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            }
        );
    }

    #[test]
    fn token_tuple_variant_ne_name() {
        assert_ne!(
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::TupleVariant {
                name: "b",
                variant_index: 1,
                variant: "foo",
                len: 42
            }
        );
    }

    #[test]
    fn token_tuple_variant_ne_variant_index() {
        assert_ne!(
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::TupleVariant {
                name: "a",
                variant_index: 2,
                variant: "foo",
                len: 42
            }
        );
    }

    #[test]
    fn token_tuple_variant_ne_variant() {
        assert_ne!(
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "bar",
                len: 42
            }
        );
    }

    #[test]
    fn token_tuple_variant_ne_len() {
        assert_ne!(
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::TupleVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 43
            }
        );
    }

    #[test]
    fn token_tuple_variant_end_eq() {
        assert_eq!(Token::TupleVariantEnd, Token::TupleVariantEnd);
    }

    #[test]
    fn token_map_eq_some() {
        assert_eq!(Token::Map { len: Some(42) }, Token::Map { len: Some(42) });
    }

    #[test]
    fn token_map_eq_none() {
        assert_eq!(Token::Map { len: None }, Token::Map { len: None });
    }

    #[test]
    fn token_map_ne_some() {
        assert_ne!(Token::Map { len: Some(42) }, Token::Map { len: Some(43) });
    }

    #[test]
    fn token_map_ne_some_none() {
        assert_ne!(Token::Map { len: Some(42) }, Token::Map { len: None });
    }

    #[test]
    fn token_map_end_eq() {
        assert_eq!(Token::MapEnd, Token::MapEnd);
    }

    #[test]
    fn token_struct_eq() {
        assert_eq!(
            Token::Struct { name: "a", len: 42 },
            Token::Struct { name: "a", len: 42 }
        );
    }

    #[test]
    fn token_struct_ne_name() {
        assert_ne!(
            Token::Struct { name: "a", len: 42 },
            Token::Struct { name: "b", len: 42 }
        );
    }

    #[test]
    fn token_struct_ne_len() {
        assert_ne!(
            Token::Struct { name: "a", len: 42 },
            Token::Struct { name: "a", len: 43 }
        );
    }

    #[test]
    fn token_struct_end_eq() {
        assert_eq!(Token::StructEnd, Token::StructEnd);
    }

    #[test]
    fn token_struct_variant_ne_name() {
        assert_ne!(
            Token::StructVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::StructVariant {
                name: "b",
                variant_index: 1,
                variant: "foo",
                len: 42
            }
        );
    }

    #[test]
    fn token_struct_variant_ne_variant_index() {
        assert_ne!(
            Token::StructVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::StructVariant {
                name: "a",
                variant_index: 2,
                variant: "foo",
                len: 42
            }
        );
    }

    #[test]
    fn token_struct_variant_ne_variant() {
        assert_ne!(
            Token::StructVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::StructVariant {
                name: "a",
                variant_index: 1,
                variant: "bar",
                len: 42
            }
        );
    }

    #[test]
    fn token_struct_variant_ne_len() {
        assert_ne!(
            Token::StructVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 42
            },
            Token::StructVariant {
                name: "a",
                variant_index: 1,
                variant: "foo",
                len: 43
            }
        );
    }

    #[test]
    fn token_struct_variant_end_eq() {
        assert_eq!(Token::StructVariantEnd, Token::StructVariantEnd);
    }

    #[test]
    fn token_field_eq() {
        assert_eq!(Token::Field("a"), Token::Field("a"));
    }

    #[test]
    fn token_field_ne() {
        assert_ne!(Token::Field("a"), Token::Field("b"));
    }

    #[test]
    fn token_skipped_field_eq() {
        assert_eq!(Token::SkippedField("a"), Token::SkippedField("a"));
    }

    #[test]
    fn token_skipped_field_ne() {
        assert_ne!(Token::SkippedField("a"), Token::SkippedField("b"));
    }

    #[test]
    fn token_variant_ne() {
        assert_ne!(Token::Bool(true), Token::U16(42));
    }

    #[test]
    fn token_unordered_eq_same_order() {
        assert_eq!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]])
        );
    }

    #[test]
    fn token_unordered_eq_different_order() {
        assert_eq!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::U8(42)], &[Token::Bool(true)]])
        );
    }

    #[test]
    fn token_unordered_ne_same_order() {
        assert_ne!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(43)]])
        );
    }

    #[test]
    fn token_unordered_ne_different_order() {
        assert_ne!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::U8(42)], &[Token::Bool(false)]])
        );
    }

    #[test]
    fn token_unordered_ne_len_shorter() {
        assert_ne!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::Bool(true)]])
        );
    }

    #[test]
    fn token_unordered_ne_len_longer() {
        assert_ne!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::Bool(true)], &[Token::Bool(true)], &[Token::U8(42)]])
        );
    }

    #[test]
    fn token_unordered_ne_different_variant() {
        assert_ne!(
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
            Token::Unordered(&[&[Token::Bool(true)], &[Token::U16(42)]])
        );
    }

    #[test]
    fn token_unordered_eq_nested() {
        assert_eq!(
            Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]])]
            ]),
            Token::Unordered(&[
                &[Token::Unordered(&[&[Token::U8(42)], &[Token::Bool(true)]])],
                &[Token::Bool(true)]
            ])
        );
    }

    #[test]
    fn tokens_bool_eq() {
        assert_eq!(
            Tokens(vec![Token::Bool(true)]),
            Tokens(vec![Token::Bool(true)])
        );
    }

    #[test]
    fn tokens_bool_ne() {
        assert_ne!(
            Tokens(vec![Token::Bool(true)]),
            Tokens(vec![Token::Bool(false)])
        );
    }

    #[test]
    fn tokens_variant_ne() {
        assert_ne!(
            Tokens(vec![Token::Bool(true)]),
            Tokens(vec![Token::U16(42)])
        );
    }

    #[test]
    fn tokens_empty_eq() {
        assert_eq!(Tokens(vec![]), Tokens(vec![]));
    }

    #[test]
    fn tokens_multiple_eq() {
        assert_eq!(
            Tokens(vec![Token::Bool(true), Token::U8(42)]),
            Tokens(vec![Token::Bool(true), Token::U8(42)])
        );
    }

    #[test]
    fn tokens_multiple_ne_values() {
        assert_ne!(
            Tokens(vec![Token::Bool(true), Token::U8(42)]),
            Tokens(vec![Token::Bool(false), Token::U8(42)])
        );
    }

    #[test]
    fn tokens_multiple_ne_shorter() {
        assert_ne!(
            Tokens(vec![Token::Bool(true), Token::U8(42)]),
            Tokens(vec![Token::Bool(true)])
        );
    }

    #[test]
    fn tokens_multiple_ne_longer() {
        assert_ne!(
            Tokens(vec![Token::Bool(true), Token::U8(42)]),
            Tokens(vec![Token::Bool(true), Token::U8(42), Token::U8(42)])
        );
    }

    #[test]
    fn tokens_unordered_both_sides_eq() {
        assert_eq!(
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::U8(42)]
            ])]),
            Tokens(vec![Token::Unordered(&[
                &[Token::U8(42)],
                &[Token::Bool(true)]
            ])])
        );
    }

    #[test]
    fn tokens_unordered_both_sides_ne() {
        assert_ne!(
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::U8(42)]
            ])]),
            Tokens(vec![Token::Unordered(&[
                &[Token::U8(42)],
                &[Token::Bool(false)]
            ])])
        );
    }

    #[test]
    fn tokens_unordered_left_eq_same_order() {
        assert_eq!(
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::U8(42)]
            ])]),
            Tokens(vec![Token::Bool(true), Token::U8(42)])
        );
    }

    #[test]
    fn tokens_unordered_left_eq_different_order() {
        assert_eq!(
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::U8(42)]
            ])]),
            Tokens(vec![Token::U8(42), Token::Bool(true)])
        );
    }

    #[test]
    fn tokens_unordered_left_eq_within_other_tokens() {
        assert_eq!(
            Tokens(vec![
                Token::Char('a'),
                Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
                Token::I16(-42)
            ]),
            Tokens(vec![
                Token::Char('a'),
                Token::U8(42),
                Token::Bool(true),
                Token::I16(-42)
            ])
        );
    }

    #[test]
    fn tokens_unordered_left_eq_multiple_tokens() {
        assert_eq!(
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true), Token::Char('a')],
                &[Token::U8(42)]
            ])]),
            Tokens(vec![Token::U8(42), Token::Bool(true), Token::Char('a')])
        );
    }

    #[test]
    fn tokens_unordered_left_ne_empty() {
        assert_ne!(
            Tokens(vec![Token::Unordered(&[])]),
            Tokens(vec![Token::Bool(true)])
        );
    }

    #[test]
    fn tokens_unordered_left_ne_variant() {
        assert_ne!(
            Tokens(vec![Token::Unordered(&[&[Token::I8(42)]])]),
            Tokens(vec![Token::Bool(true)])
        );
    }

    #[test]
    fn tokens_unordered_left_ne_value() {
        assert_ne!(
            Tokens(vec![Token::Unordered(&[&[Token::Bool(false)]])]),
            Tokens(vec![Token::Bool(true)])
        );
    }

    #[test]
    fn tokens_unordered_right_eq_same_order() {
        assert_eq!(
            Tokens(vec![Token::Bool(true), Token::U8(42)]),
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::U8(42)]
            ])]),
        );
    }

    #[test]
    fn tokens_unordered_right_eq_different_order() {
        assert_eq!(
            Tokens(vec![Token::U8(42), Token::Bool(true)]),
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::U8(42)]
            ])]),
        );
    }

    #[test]
    fn tokens_unordered_right_eq_within_other_tokens() {
        assert_eq!(
            Tokens(vec![
                Token::Char('a'),
                Token::U8(42),
                Token::Bool(true),
                Token::I16(-42)
            ]),
            Tokens(vec![
                Token::Char('a'),
                Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
                Token::I16(-42)
            ]),
        );
    }

    #[test]
    fn tokens_unordered_right_eq_multiple_tokens() {
        assert_eq!(
            Tokens(vec![Token::U8(42), Token::Bool(true), Token::Char('a')]),
            Tokens(vec![Token::Unordered(&[
                &[Token::Bool(true), Token::Char('a')],
                &[Token::U8(42)]
            ])]),
        );
    }

    #[test]
    fn tokens_unordered_right_ne_empty() {
        assert_ne!(
            Tokens(vec![Token::Bool(true)]),
            Tokens(vec![Token::Unordered(&[])]),
        );
    }

    #[test]
    fn tokens_unordered_right_ne_variant() {
        assert_ne!(
            Tokens(vec![Token::Bool(true)]),
            Tokens(vec![Token::Unordered(&[&[Token::I8(42)]])]),
        );
    }

    #[test]
    fn tokens_unordered_right_ne_value() {
        assert_ne!(
            Tokens(vec![Token::Bool(true)]),
            Tokens(vec![Token::Unordered(&[&[Token::Bool(false)]])]),
        );
    }

    #[test]
    fn unexpected_from_token_bool() {
        assert_eq!(Unexpected::from(&Token::Bool(true)), Unexpected::Bool(true))
    }

    #[test]
    fn unexpected_from_token_i8() {
        assert_eq!(Unexpected::from(&Token::I8(42)), Unexpected::Signed(42))
    }

    #[test]
    fn unexpected_from_token_i16() {
        assert_eq!(Unexpected::from(&Token::I16(42)), Unexpected::Signed(42))
    }

    #[test]
    fn unexpected_from_token_i32() {
        assert_eq!(Unexpected::from(&Token::I32(42)), Unexpected::Signed(42))
    }

    #[test]
    fn unexpected_from_token_i64() {
        assert_eq!(Unexpected::from(&Token::I64(42)), Unexpected::Signed(42))
    }

    #[cfg(has_i128)]
    #[test]
    fn unexpected_from_token_i128() {
        assert_eq!(
            Unexpected::from(&Token::I128(42)),
            Unexpected::Other("i128")
        )
    }

    #[test]
    fn unexpected_from_token_u8() {
        assert_eq!(Unexpected::from(&Token::U8(42)), Unexpected::Unsigned(42))
    }

    #[test]
    fn unexpected_from_token_u16() {
        assert_eq!(Unexpected::from(&Token::U16(42)), Unexpected::Unsigned(42))
    }

    #[test]
    fn unexpected_from_token_u32() {
        assert_eq!(Unexpected::from(&Token::U32(42)), Unexpected::Unsigned(42))
    }

    #[test]
    fn unexpected_from_token_u64() {
        assert_eq!(Unexpected::from(&Token::U64(42)), Unexpected::Unsigned(42))
    }

    #[cfg(has_i128)]
    #[test]
    fn unexpected_from_token_u128() {
        assert_eq!(
            Unexpected::from(&Token::U128(42)),
            Unexpected::Other("u128")
        )
    }

    #[test]
    fn unexpected_from_token_f32() {
        assert_eq!(Unexpected::from(&Token::F32(42.)), Unexpected::Float(42.))
    }

    #[test]
    fn unexpected_from_token_f64() {
        assert_eq!(Unexpected::from(&Token::F64(42.)), Unexpected::Float(42.))
    }

    #[test]
    fn unexpected_from_token_char() {
        assert_eq!(Unexpected::from(&Token::Char('a')), Unexpected::Char('a'))
    }

    #[test]
    fn unexpected_from_token_str() {
        assert_eq!(
            Unexpected::from(&Token::Str("foo".to_owned())),
            Unexpected::Str("foo")
        )
    }

    #[test]
    fn unexpected_from_token_bytes() {
        assert_eq!(
            Unexpected::from(&Token::Bytes(b"foo".to_vec())),
            Unexpected::Bytes(b"foo")
        )
    }

    #[test]
    fn unexpected_from_token_some() {
        assert_eq!(Unexpected::from(&Token::Some), Unexpected::Option)
    }

    #[test]
    fn unexpected_from_token_none() {
        assert_eq!(Unexpected::from(&Token::None), Unexpected::Option)
    }

    #[test]
    fn unexpected_from_token_unit() {
        assert_eq!(Unexpected::from(&Token::Unit), Unexpected::Unit)
    }

    #[test]
    fn unexpected_from_token_unit_struct() {
        assert_eq!(
            Unexpected::from(&Token::UnitStruct { name: "foo" }),
            Unexpected::Unit
        )
    }

    #[test]
    fn unexpected_from_token_unit_variant() {
        assert_eq!(
            Unexpected::from(&Token::UnitVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar"
            }),
            Unexpected::UnitVariant
        )
    }

    #[test]
    fn unexpected_from_token_newtype_struct() {
        assert_eq!(
            Unexpected::from(&Token::NewtypeStruct { name: "foo" }),
            Unexpected::NewtypeStruct
        )
    }

    #[test]
    fn unexpected_from_token_newtype_variant() {
        assert_eq!(
            Unexpected::from(&Token::NewtypeVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar"
            }),
            Unexpected::NewtypeVariant
        )
    }

    #[test]
    fn unexpected_from_token_seq() {
        assert_eq!(Unexpected::from(&Token::Seq { len: None }), Unexpected::Seq)
    }

    #[test]
    fn unexpected_from_token_tuple() {
        assert_eq!(Unexpected::from(&Token::Tuple { len: 0 }), Unexpected::Seq)
    }

    #[test]
    fn unexpected_from_token_seq_end() {
        assert_eq!(
            Unexpected::from(&Token::SeqEnd),
            Unexpected::Other("SeqEnd")
        )
    }

    #[test]
    fn unexpected_from_token_tuple_end() {
        assert_eq!(
            Unexpected::from(&Token::TupleEnd),
            Unexpected::Other("TupleEnd")
        )
    }

    #[test]
    fn unexpected_from_token_tuple_struct() {
        assert_eq!(
            Unexpected::from(&Token::TupleStruct {
                name: "foo",
                len: 0
            }),
            Unexpected::Other("TupleStruct")
        )
    }

    #[test]
    fn unexpected_from_token_tuple_struct_end() {
        assert_eq!(
            Unexpected::from(&Token::TupleStructEnd),
            Unexpected::Other("TupleStructEnd")
        )
    }

    #[test]
    fn unexpected_from_token_tuple_variant() {
        assert_eq!(
            Unexpected::from(&Token::TupleVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar",
                len: 0
            }),
            Unexpected::TupleVariant
        )
    }

    #[test]
    fn unexpected_from_token_tuple_variant_end() {
        assert_eq!(
            Unexpected::from(&Token::TupleVariantEnd),
            Unexpected::Other("TupleVariantEnd")
        )
    }

    #[test]
    fn unexpected_from_token_map() {
        assert_eq!(Unexpected::from(&Token::Map { len: None }), Unexpected::Map)
    }

    #[test]
    fn unexpected_from_token_map_end() {
        assert_eq!(
            Unexpected::from(&Token::MapEnd),
            Unexpected::Other("MapEnd")
        )
    }

    #[test]
    fn unexpected_from_token_field() {
        assert_eq!(
            Unexpected::from(&Token::Field("foo")),
            Unexpected::Other("Field")
        )
    }

    #[test]
    fn unexpected_from_token_skipped_field() {
        assert_eq!(
            Unexpected::from(&Token::SkippedField("foo")),
            Unexpected::Other("SkippedField")
        )
    }

    #[test]
    fn unexpected_from_token_struct() {
        assert_eq!(
            Unexpected::from(&Token::Struct {
                name: "foo",
                len: 0
            }),
            Unexpected::Other("Struct")
        )
    }

    #[test]
    fn unexpected_from_token_struct_end() {
        assert_eq!(
            Unexpected::from(&Token::StructEnd),
            Unexpected::Other("StructEnd")
        )
    }

    #[test]
    fn unexpected_from_token_struct_variant() {
        assert_eq!(
            Unexpected::from(&Token::StructVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar",
                len: 0
            }),
            Unexpected::StructVariant
        )
    }

    #[test]
    fn unexpected_from_token_struct_variant_end() {
        assert_eq!(
            Unexpected::from(&Token::StructVariantEnd),
            Unexpected::Other("StructVariantEnd")
        )
    }

    #[test]
    fn unexpected_from_token_unordered() {
        assert_eq!(
            Unexpected::from(&Token::Unordered(&[])),
            Unexpected::Other("unordered tokens")
        )
    }

    #[test]
    fn iter_empty() {
        let mut iter = Iter::new(Tokens(Vec::new()));

        assert_none!(iter.next());
    }

    #[test]
    fn iter_one_token() {
        let mut iter = Iter::new(Tokens(vec![Token::Bool(true)]));

        assert_some_eq!(iter.next(), &Token::Bool(true));
        assert_none!(iter.next());
    }

    #[test]
    fn iter_multiple_tokens() {
        let mut iter = Iter::new(Tokens(vec![
            Token::Bool(true),
            Token::U64(42),
            Token::Str("foo".to_owned()),
        ]));

        assert_some_eq!(iter.next(), &Token::Bool(true));
        assert_some_eq!(iter.next(), &Token::U64(42));
        assert_some_eq!(iter.next(), &Token::Str("foo".to_owned()));
        assert_none!(iter.next());
    }

    #[test]
    fn iter_empty_debug() {
        let iter = Iter::new(Tokens(Vec::new()));

        assert_eq!(format!("{:?}", iter), "Iter([])")
    }

    #[test]
    fn iter_uniterated_debug() {
        let iter = Iter::new(Tokens(vec![
            Token::Bool(true),
            Token::U64(42),
            Token::Str("foo".to_owned()),
        ]));

        assert_eq!(
            format!("{:?}", iter),
            "Iter([Bool(true), U64(42), Str(\"foo\")])"
        )
    }

    #[test]
    fn iter_partially_iterated_debug() {
        let mut iter = Iter::new(Tokens(vec![
            Token::Bool(true),
            Token::U64(42),
            Token::Str("foo".to_owned()),
        ]));

        assert_some!(iter.next());

        assert_eq!(format!("{:?}", iter), "Iter([U64(42), Str(\"foo\")])")
    }

    #[test]
    fn iter_fully_iterated_debug() {
        let mut iter = Iter::new(Tokens(vec![
            Token::Bool(true),
            Token::U64(42),
            Token::Str("foo".to_owned()),
        ]));

        assert_some!(iter.next());
        assert_some!(iter.next());
        assert_some!(iter.next());

        assert_eq!(format!("{:?}", iter), "Iter([])")
    }
}
