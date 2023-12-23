//! Tokens representing a serialized object.
//!
//! This module provides a [`Token`] type for representing a serialized value, as well as a
//! [`Tokens`] type containing a set of `Token`s. `Tokens` are returned by a [`Serializer`] and can
//! be compared against a sequence of `Token`s to verify equality, which is useful for asserting
//! whether serialized `Tokens` are as expected.
//!
//! [`Serializer`]: crate::Serializer

use alloc::{
    slice,
    string::String,
    vec,
    vec::Vec,
};
use core::{
    fmt,
    fmt::Debug,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};
use serde::de::Unexpected;

/// A single serialized value.
///
/// A `Token` is a single serialization output produced by the [`Serializer`]. The one exception to
/// this is the [`Unordered`] variant, which contains multiple sets of tokens that can be in any
/// order. This is never produced by the `Serializer`, and is for use when comparing equality of
/// sequences of [`Token`]s.
///
/// Normally, a sequence of `Token`s are used to either compare against the output of a
/// [`Serializer`] or to be used as input to a [`Deserializer`].
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(true.serialize(&serializer), [Token::Bool(true)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i8.serialize(&serializer), [Token::I8(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i16.serialize(&serializer), [Token::I16(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i32.serialize(&serializer), [Token::I32(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i64.serialize(&serializer), [Token::I64(42)]);
    /// ```
    I64(i64),

    /// An [`i128`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42i128.serialize(&serializer), [Token::I128(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u8.serialize(&serializer), [Token::U8(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u16.serialize(&serializer), [Token::U16(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u32.serialize(&serializer), [Token::U32(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u64.serialize(&serializer), [Token::U64(42)]);
    /// ```
    U64(u64),

    /// A [`u128`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42u128.serialize(&serializer), [Token::U128(42)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42.0f32.serialize(&serializer), [Token::F32(42.0)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(42.0f64.serialize(&serializer), [Token::F64(42.0)]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!('a'.serialize(&serializer), [Token::Char('a')]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!("foo".serialize(&serializer), [Token::Str("foo".to_owned())]);
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
    /// };
    /// use serde_bytes::Bytes;
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     Bytes::new(b"foo").serialize(&serializer),
    ///     [Token::Bytes(b"foo".to_vec())]
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(Option::<()>::None.serialize(&serializer), [Token::None]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(Some(()).serialize(&serializer), [Token::Some, Token::Unit]);
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(().serialize(&serializer), [Token::Unit]);
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
    /// };
    /// # use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct UnitStruct;
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     UnitStruct.serialize(&serializer),
    ///     [Token::UnitStruct { name: "UnitStruct" }]
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
    /// };
    /// # use serde_derive::Serialize;
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
    ///     [Token::UnitVariant {
    ///         name: "Enum",
    ///         variant_index: 0,
    ///         variant: "Unit"
    ///     }]
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
    /// };
    /// # use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct NewtypeStruct(u32);
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     NewtypeStruct(42).serialize(&serializer),
    ///     [
    ///         Token::NewtypeStruct {
    ///             name: "NewtypeStruct"
    ///         },
    ///         Token::U32(42)
    ///     ]
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
    /// };
    /// # use serde_derive::Serialize;
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
    ///     [
    ///         Token::NewtypeVariant {
    ///             name: "Enum",
    ///             variant_index: 0,
    ///             variant: "Newtype"
    ///         },
    ///         Token::U32(42)
    ///     ]
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     vec![1u32, 2u32, 3u32].serialize(&serializer),
    ///     [
    ///         Token::Seq { len: Some(3) },
    ///         Token::U32(1),
    ///         Token::U32(2),
    ///         Token::U32(3),
    ///         Token::SeqEnd
    ///     ]
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     (42u32, true).serialize(&serializer),
    ///     [
    ///         Token::Tuple { len: 2 },
    ///         Token::U32(42),
    ///         Token::Bool(true),
    ///         Token::TupleEnd
    ///     ]
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
    /// };
    /// # use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct TupleStruct(u32, bool);
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// assert_ok_eq!(
    ///     TupleStruct(42u32, true).serialize(&serializer),
    ///     [
    ///         Token::TupleStruct {
    ///             name: "TupleStruct",
    ///             len: 2
    ///         },
    ///         Token::U32(42),
    ///         Token::Bool(true),
    ///         Token::TupleStructEnd
    ///     ]
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
    /// };
    /// # use serde_derive::Serialize;
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
    ///     [
    ///         Token::TupleVariant {
    ///             name: "Enum",
    ///             variant_index: 0,
    ///             variant: "Tuple",
    ///             len: 2
    ///         },
    ///         Token::U32(42),
    ///         Token::Bool(true),
    ///         Token::TupleVariantEnd
    ///     ]
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
    /// };
    ///
    /// let serializer = Serializer::builder().build();
    ///
    /// let mut map = HashMap::new();
    /// map.insert("foo", 42u32);
    ///
    /// assert_ok_eq!(
    ///     map.serialize(&serializer),
    ///     [
    ///         Token::Map { len: Some(1) },
    ///         Token::Str("foo".to_owned()),
    ///         Token::U32(42),
    ///         Token::MapEnd
    ///     ]
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
    /// };
    /// # use serde_derive::Serialize;
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
    ///     [
    ///         Token::Struct {
    ///             name: "Struct",
    ///             len: 2
    ///         },
    ///         Token::Field("foo"),
    ///         Token::U32(42),
    ///         Token::Field("bar"),
    ///         Token::Bool(true),
    ///         Token::StructEnd
    ///     ]
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
    /// };
    /// # use serde_derive::Serialize;
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
    ///     [
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
    ///     ]
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
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use hashbrown::HashMap;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     Serializer,
    ///     Token,
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
    ///     [
    ///         Token::Map { len: Some(3) },
    ///         Token::Unordered(&[
    ///             &[Token::Char('a'), Token::U32(1)],
    ///             &[Token::Char('b'), Token::U32(2)],
    ///             &[Token::Char('c'), Token::U32(3)]
    ///         ]),
    ///         Token::MapEnd
    ///     ]
    /// );
    /// ```
    ///
    /// [`HashSet`]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html
    /// [`Serializer`]: crate::Serializer
    Unordered(&'static [&'static [Token]]),
}

/// An enumeration of all tokens that can be emitted by the [`Serializer`].
///
/// [`Serializer`]: crate::Serializer
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CanonicalToken {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    None,
    Some,
    Unit,
    UnitStruct {
        name: &'static str,
    },
    UnitVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    },
    NewtypeStruct {
        name: &'static str,
    },
    NewtypeVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    },
    Seq {
        len: Option<usize>,
    },
    SeqEnd,
    Tuple {
        len: usize,
    },
    TupleEnd,
    TupleStruct {
        name: &'static str,
        len: usize,
    },
    TupleStructEnd,
    TupleVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    },
    TupleVariantEnd,
    Map {
        len: Option<usize>,
    },
    MapEnd,
    Field(&'static str),
    SkippedField(&'static str),
    Struct {
        name: &'static str,
        len: usize,
    },
    StructEnd,
    StructVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    },
    StructVariantEnd,
}

pub(crate) struct UnorderedTokens(pub(crate) &'static [&'static [Token]]);

impl TryFrom<Token> for CanonicalToken {
    type Error = UnorderedTokens;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Bool(value) => Ok(CanonicalToken::Bool(value)),
            Token::I8(value) => Ok(CanonicalToken::I8(value)),
            Token::I16(value) => Ok(CanonicalToken::I16(value)),
            Token::I32(value) => Ok(CanonicalToken::I32(value)),
            Token::I64(value) => Ok(CanonicalToken::I64(value)),
            Token::I128(value) => Ok(CanonicalToken::I128(value)),
            Token::U8(value) => Ok(CanonicalToken::U8(value)),
            Token::U16(value) => Ok(CanonicalToken::U16(value)),
            Token::U32(value) => Ok(CanonicalToken::U32(value)),
            Token::U64(value) => Ok(CanonicalToken::U64(value)),
            Token::U128(value) => Ok(CanonicalToken::U128(value)),
            Token::F32(value) => Ok(CanonicalToken::F32(value)),
            Token::F64(value) => Ok(CanonicalToken::F64(value)),
            Token::Char(value) => Ok(CanonicalToken::Char(value)),
            Token::Str(value) => Ok(CanonicalToken::Str(value)),
            Token::Bytes(value) => Ok(CanonicalToken::Bytes(value)),
            Token::None => Ok(CanonicalToken::None),
            Token::Some => Ok(CanonicalToken::Some),
            Token::Unit => Ok(CanonicalToken::Unit),
            Token::UnitStruct { name } => Ok(CanonicalToken::UnitStruct { name }),
            Token::UnitVariant {
                name,
                variant_index,
                variant,
            } => Ok(CanonicalToken::UnitVariant {
                name,
                variant_index,
                variant,
            }),
            Token::NewtypeStruct { name } => Ok(CanonicalToken::NewtypeStruct { name }),
            Token::NewtypeVariant {
                name,
                variant_index,
                variant,
            } => Ok(CanonicalToken::NewtypeVariant {
                name,
                variant_index,
                variant,
            }),
            Token::Seq { len } => Ok(CanonicalToken::Seq { len }),
            Token::SeqEnd => Ok(CanonicalToken::SeqEnd),
            Token::Tuple { len } => Ok(CanonicalToken::Tuple { len }),
            Token::TupleEnd => Ok(CanonicalToken::TupleEnd),
            Token::TupleStruct { name, len } => Ok(CanonicalToken::TupleStruct { name, len }),
            Token::TupleStructEnd => Ok(CanonicalToken::TupleStructEnd),
            Token::TupleVariant {
                name,
                variant_index,
                variant,
                len,
            } => Ok(CanonicalToken::TupleVariant {
                name,
                variant_index,
                variant,
                len,
            }),
            Token::TupleVariantEnd => Ok(CanonicalToken::TupleVariantEnd),
            Token::Map { len } => Ok(CanonicalToken::Map { len }),
            Token::MapEnd => Ok(CanonicalToken::MapEnd),
            Token::Field(value) => Ok(CanonicalToken::Field(value)),
            Token::SkippedField(value) => Ok(CanonicalToken::SkippedField(value)),
            Token::Struct { name, len } => Ok(CanonicalToken::Struct { name, len }),
            Token::StructEnd => Ok(CanonicalToken::StructEnd),
            Token::StructVariant {
                name,
                variant_index,
                variant,
                len,
            } => Ok(CanonicalToken::StructVariant {
                name,
                variant_index,
                variant,
                len,
            }),
            Token::StructVariantEnd => Ok(CanonicalToken::StructVariantEnd),
            Token::Unordered(tokens) => Err(UnorderedTokens(tokens)),
        }
    }
}

impl From<CanonicalToken> for Token {
    fn from(token: CanonicalToken) -> Self {
        match token {
            CanonicalToken::Bool(value) => Token::Bool(value),
            CanonicalToken::I8(value) => Token::I8(value),
            CanonicalToken::I16(value) => Token::I16(value),
            CanonicalToken::I32(value) => Token::I32(value),
            CanonicalToken::I64(value) => Token::I64(value),
            CanonicalToken::I128(value) => Token::I128(value),
            CanonicalToken::U8(value) => Token::U8(value),
            CanonicalToken::U16(value) => Token::U16(value),
            CanonicalToken::U32(value) => Token::U32(value),
            CanonicalToken::U64(value) => Token::U64(value),
            CanonicalToken::U128(value) => Token::U128(value),
            CanonicalToken::F32(value) => Token::F32(value),
            CanonicalToken::F64(value) => Token::F64(value),
            CanonicalToken::Char(value) => Token::Char(value),
            CanonicalToken::Str(value) => Token::Str(value),
            CanonicalToken::Bytes(value) => Token::Bytes(value),
            CanonicalToken::None => Token::None,
            CanonicalToken::Some => Token::Some,
            CanonicalToken::Unit => Token::Unit,
            CanonicalToken::UnitStruct { name } => Token::UnitStruct { name },
            CanonicalToken::UnitVariant {
                name,
                variant_index,
                variant,
            } => Token::UnitVariant {
                name,
                variant_index,
                variant,
            },
            CanonicalToken::NewtypeStruct { name } => Token::NewtypeStruct { name },
            CanonicalToken::NewtypeVariant {
                name,
                variant_index,
                variant,
            } => Token::NewtypeVariant {
                name,
                variant_index,
                variant,
            },
            CanonicalToken::Seq { len } => Token::Seq { len },
            CanonicalToken::SeqEnd => Token::SeqEnd,
            CanonicalToken::Tuple { len } => Token::Tuple { len },
            CanonicalToken::TupleEnd => Token::TupleEnd,
            CanonicalToken::TupleStruct { name, len } => Token::TupleStruct { name, len },
            CanonicalToken::TupleStructEnd => Token::TupleStructEnd,
            CanonicalToken::TupleVariant {
                name,
                variant_index,
                variant,
                len,
            } => Token::TupleVariant {
                name,
                variant_index,
                variant,
                len,
            },
            CanonicalToken::TupleVariantEnd => Token::TupleVariantEnd,
            CanonicalToken::Map { len } => Token::Map { len },
            CanonicalToken::MapEnd => Token::MapEnd,
            CanonicalToken::Field(value) => Token::Field(value),
            CanonicalToken::SkippedField(value) => Token::SkippedField(value),
            CanonicalToken::Struct { name, len } => Token::Struct { name, len },
            CanonicalToken::StructEnd => Token::StructEnd,
            CanonicalToken::StructVariant {
                name,
                variant_index,
                variant,
                len,
            } => Token::StructVariant {
                name,
                variant_index,
                variant,
                len,
            },
            CanonicalToken::StructVariantEnd => Token::StructVariantEnd,
        }
    }
}

impl<'a> From<&'a CanonicalToken> for Unexpected<'a> {
    fn from(token: &'a CanonicalToken) -> Self {
        match token {
            CanonicalToken::Bool(v) => Unexpected::Bool(*v),
            CanonicalToken::I8(v) => Unexpected::Signed((*v).into()),
            CanonicalToken::I16(v) => Unexpected::Signed((*v).into()),
            CanonicalToken::I32(v) => Unexpected::Signed((*v).into()),
            CanonicalToken::I64(v) => Unexpected::Signed(*v),
            CanonicalToken::I128(..) => Unexpected::Other("i128"),
            CanonicalToken::U8(v) => Unexpected::Unsigned((*v).into()),
            CanonicalToken::U16(v) => Unexpected::Unsigned((*v).into()),
            CanonicalToken::U32(v) => Unexpected::Unsigned((*v).into()),
            CanonicalToken::U64(v) => Unexpected::Unsigned(*v),
            CanonicalToken::U128(..) => Unexpected::Other("u128"),
            CanonicalToken::F32(v) => Unexpected::Float((*v).into()),
            CanonicalToken::F64(v) => Unexpected::Float(*v),
            CanonicalToken::Char(v) => Unexpected::Char(*v),
            CanonicalToken::Str(v) => Unexpected::Str(v),
            CanonicalToken::Bytes(v) => Unexpected::Bytes(v),
            CanonicalToken::Some | CanonicalToken::None => Unexpected::Option,
            CanonicalToken::Unit | CanonicalToken::UnitStruct { .. } => Unexpected::Unit,
            CanonicalToken::UnitVariant { .. } => Unexpected::UnitVariant,
            CanonicalToken::NewtypeStruct { .. } => Unexpected::NewtypeStruct,
            CanonicalToken::NewtypeVariant { .. } => Unexpected::NewtypeVariant,
            CanonicalToken::Seq { .. } | CanonicalToken::Tuple { .. } => Unexpected::Seq,
            CanonicalToken::SeqEnd => Unexpected::Other("SeqEnd"),
            CanonicalToken::TupleEnd => Unexpected::Other("TupleEnd"),
            CanonicalToken::TupleStruct { .. } => Unexpected::Other("TupleStruct"),
            CanonicalToken::TupleStructEnd => Unexpected::Other("TupleStructEnd"),
            CanonicalToken::TupleVariant { .. } => Unexpected::TupleVariant,
            CanonicalToken::TupleVariantEnd => Unexpected::Other("TupleVariantEnd"),
            CanonicalToken::Map { .. } => Unexpected::Map,
            CanonicalToken::MapEnd => Unexpected::Other("MapEnd"),
            CanonicalToken::Field(..) => Unexpected::Other("Field"),
            CanonicalToken::SkippedField(..) => Unexpected::Other("SkippedField"),
            CanonicalToken::Struct { .. } => Unexpected::Other("Struct"),
            CanonicalToken::StructEnd => Unexpected::Other("StructEnd"),
            CanonicalToken::StructVariant { .. } => Unexpected::StructVariant,
            CanonicalToken::StructVariantEnd => Unexpected::Other("StructVariantEnd"),
        }
    }
}

/// A sequence of [`Token`]s output by a [`Serializer`].
///
/// `Tokens` can be compared with any other sequence of `Token`s to assert that the serialized
/// values are as expected.
///
/// # Examples
///
/// `Tokens` are output from a [`Serializer`] and can be compared against a sequence of `Token`s.
///
/// ## Serialization
///
/// ``` rust
/// use claims::assert_ok_eq;
/// use serde::Serialize;
/// use serde_assert::{
///     Serializer,
///     Token,
/// };
///
/// let serializer = Serializer::builder().build();
///
/// assert_ok_eq!(true.serialize(&serializer), [Token::Bool(true)]);
/// ```
///
/// ## Deserialization
///
/// `Tokens` obtained from a [`Serializer`] can be used as input to a [`Deserializer`].
///
/// ``` rust
/// use claims::{
///     assert_ok,
///     assert_ok_eq,
/// };
/// use serde::{
///     Deserialize,
///     Serialize,
/// };
/// use serde_assert::{
///     Deserializer,
///     Serializer,
///     Token,
/// };
///
/// let serializer = Serializer::builder().build();
/// let mut deserializer = Deserializer::builder(assert_ok!(true.serialize(&serializer))).build();
///
/// assert_ok_eq!(bool::deserialize(&mut deserializer), true);
/// ```
///
/// [`Deserializer`]: crate::Deserializer
/// [`Serializer`]: crate::Serializer
#[derive(Clone, Debug)]
pub struct Tokens(pub(crate) Vec<CanonicalToken>);

/// A specific context when traversing through a possible path in the given tokens.
///
/// This will either be in the main iterator of tokens (`Iterator`) or in the context of a
/// (possibly nested) `Unordered` token.
#[derive(Clone, Debug)]
enum StateContext<'a, T> {
    Iterator(T),
    Unordered {
        current: slice::Iter<'a, Token>,
        remaining: Vec<&'static [Token]>,
    },
}

/// A current state when traversing through a possible path in the given tokens.
#[derive(Clone, Debug)]
struct State<'a, T>(Vec<StateContext<'a, T>>);

impl<'a, T> State<'a, T>
where
    T: Clone + Iterator<Item = &'a Token>,
{
    // TODO: Want to remove the clone requirement, and just share the same base iterator for all
    // branching paths. Maybe there's a better way to structure this?

    /// Splits along several possible paths in an unordered set of tokens, creating a new `State`
    /// for each path.
    ///
    /// The other untraveled paths are aded into the `remaining` field of `StateContext::Unordered`
    /// for future processing.
    ///
    /// This function also steps into each path using the `input` token.
    fn split(self, paths: &[&'static [Token]], input: &CanonicalToken) -> Vec<Self> {
        (0..paths.len())
            .map(move |index| {
                let mut new_state = self.clone();
                new_state.0.push(StateContext::Unordered {
                    current: paths[index].iter(),
                    remaining: paths
                        .iter()
                        .enumerate()
                        .filter_map(|(i, tokens)| if i == index { None } else { Some(*tokens) })
                        .collect(),
                });
                new_state
            })
            .flat_map(|state| state.feed(input))
            .collect()
    }

    /// Travels along all possible paths through `token`.
    ///
    /// This will branch if the next token is `Unordered`. In other cases, it just returns 0 or 1
    /// state.
    fn feed(mut self, input: &CanonicalToken) -> Vec<Self> {
        // Obtain the next token from the contexts.
        //
        // This is done by obtaining the token from the last context that contains tokens.
        // If a context does not contain tokens, it is popped.
        // If it is in the middle of an unordered set, we create new states with the remaining
        // tokens and process those.
        if let Some(context) = self.0.last_mut() {
            match context {
                StateContext::Iterator(tokens) => {
                    if let Some(token) = tokens.next() {
                        match CanonicalToken::try_from(token.clone()) {
                            Ok(canonical_token) => {
                                // Compare tokens.
                                if *input == canonical_token {
                                    vec![self]
                                } else {
                                    Vec::new()
                                }
                            }
                            Err(UnorderedTokens(unordered_tokens)) => {
                                // Split.
                                if unordered_tokens.is_empty() {
                                    // This unordered tokens is empty, so we move on to processing
                                    // the next token.
                                    self.feed(input)
                                } else {
                                    self.split(unordered_tokens, input)
                                }
                            }
                        }
                    } else {
                        // No tokens left. Pop this iterator and reprocess the token.
                        self.0.pop();
                        self.feed(input)
                    }
                }
                StateContext::Unordered {
                    current: tokens,
                    remaining,
                } => {
                    if let Some(token) = tokens.next() {
                        match CanonicalToken::try_from(token.clone()) {
                            Ok(canonical_token) => {
                                // Compare tokens.
                                if *input == canonical_token {
                                    vec![self]
                                } else {
                                    Vec::new()
                                }
                            }
                            Err(UnorderedTokens(unordered_tokens)) => {
                                // Split.
                                if unordered_tokens.is_empty() {
                                    // This unordered tokens is empty, so we move on to processing
                                    // the next token.
                                    self.feed(input)
                                } else {
                                    self.split(unordered_tokens, input)
                                }
                            }
                        }
                    } else {
                        // No tokens left. Pop, possibly split, and reprocess the token.
                        let remaining = remaining.clone();
                        self.0.pop();
                        if remaining.is_empty() {
                            self.feed(input)
                        } else {
                            // For each remaining, create a new state with it as the current.
                            self.split(remaining.as_slice(), input)
                        }
                    }
                }
            }
        } else {
            // There are no more tokens, so we return no new states.
            Vec::new()
        }
    }

    /// Check whether there are any remaining `CanonicalToken`s to be extracted.
    ///
    /// This will consume tokens, so don't call this unless you don't care about preserving the
    /// remaining tokens.
    fn is_empty(&mut self) -> bool {
        if let Some(context) = self.0.last_mut() {
            match context {
                StateContext::Iterator(tokens) => {
                    if let Some(token) = tokens.next() {
                        match CanonicalToken::try_from(token.clone()) {
                            Ok(_) => {
                                // Found a token, so not empty.
                                false
                            }
                            Err(UnorderedTokens(unordered_tokens)) => {
                                // We only need to check if there is at least one canonical token
                                // contained here.
                                if let Some((first, remaining)) = unordered_tokens.split_first() {
                                    self.0.push(StateContext::Unordered {
                                        current: first.iter(),
                                        remaining: remaining.to_vec(),
                                    });
                                    self.0.is_empty()
                                } else {
                                    // This unordered tokens is empty, so proceed to the next token.
                                    self.is_empty()
                                }
                            }
                        }
                    } else {
                        // No tokens left in this context; pop it and move to the next one.
                        self.0.pop();
                        self.is_empty()
                    }
                }
                StateContext::Unordered {
                    current: tokens,
                    remaining,
                } => {
                    if let Some(token) = tokens.next() {
                        match CanonicalToken::try_from(token.clone()) {
                            Ok(_) => {
                                // Found a token, so not empty.
                                false
                            }
                            Err(UnorderedTokens(unordered_tokens)) => {
                                // We only need to check if there is at least one canonical token
                                // contained here.
                                if let Some((first, remaining)) = unordered_tokens.split_first() {
                                    self.0.push(StateContext::Unordered {
                                        current: first.iter(),
                                        remaining: remaining.to_vec(),
                                    });
                                    self.0.is_empty()
                                } else {
                                    // This unordered tokens is empty, so proceed to the next token.
                                    self.is_empty()
                                }
                            }
                        }
                    } else {
                        // No tokens left. Pop, proceed down another remaining unordered path if
                        // possible, and reprocess the token.
                        let remaining = remaining.clone();
                        self.0.pop();
                        if let Some((first, remaining)) = remaining.split_first() {
                            self.0.push(StateContext::Unordered {
                                current: first.iter(),
                                remaining: remaining.to_vec(),
                            });
                            self.0.is_empty()
                        } else {
                            // No more unordered tokens, so proceed to the next context.
                            self.is_empty()
                        }
                    }
                }
            }
        } else {
            // No contexts left to check. This means there are no more tokens, and therefore the
            // state is empty.
            true
        }
    }
}

impl<T> PartialEq<T> for Tokens
where
    for<'a> &'a T: IntoIterator<Item = &'a Token>,
{
    fn eq(&self, other: &T) -> bool {
        let mut states = vec![State(vec![StateContext::Iterator(
            other.into_iter().collect::<Vec<_>>().into_iter(),
        )])];

        for token in &self.0 {
            let mut new_states = Vec::new();
            for state in states {
                new_states.extend(state.feed(token));
            }
            states = new_states;
        }

        if let Some(state) = states.first_mut() {
            // Verify whether any tokens are left in `other`.
            state.is_empty()
        } else {
            // No states, which means no path through `other` could be found.
            false
        }
    }
}

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            token_iter: self.0.into_iter(),
        }
    }
}

/// An iterator that moves [`Token`]s out of a [`Tokens`] `struct`.
///
/// This `struct` is created by the [`into_iter()`] method on `Tokens` (provided by the
/// [`IntoIterator`] trait).
///
/// [`into_iter()`]: IntoIterator::into_iter()
pub struct IntoIter {
    token_iter: vec::IntoIter<CanonicalToken>,
}

impl Iterator for IntoIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_iter.next().map(From::from)
    }
}

/// An iterator over tokens.
///
/// This iterator owns the tokens, iterating over references to them.
pub(crate) struct OwningIter<'a> {
    /// A pointer to the entire buffer that is owned by this struct.
    ///
    /// Immutable references to the `Token`s in this buffer can exist within the lifetime `'a`.
    buf: NonNull<CanonicalToken>,
    /// A pointer to the current position in iteration.
    ptr: *const CanonicalToken,
    /// A pointer to the end of the allocated buffer.
    end: *const CanonicalToken,
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

impl OwningIter<'_> {
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
    fn as_slice(&self) -> &[CanonicalToken] {
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

impl<'a> Iterator for OwningIter<'a> {
    type Item = &'a CanonicalToken;

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

impl Debug for OwningIter<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("OwningIter")
            .field(&self.as_slice())
            .finish()
    }
}

impl Drop for OwningIter<'_> {
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
        CanonicalToken,
        OwningIter,
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
        assert_matches,
        assert_none,
        assert_some,
        assert_some_eq,
    };
    use serde::de::Unexpected;

    #[test]
    fn tokens_bool_eq() {
        assert_eq!(
            Tokens(vec![CanonicalToken::Bool(true)]),
            [Token::Bool(true)]
        );
    }

    #[test]
    fn tokens_bool_ne() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true)]),
            [Token::Bool(false)]
        );
    }

    #[test]
    fn tokens_variant_ne() {
        assert_ne!(Tokens(vec![CanonicalToken::Bool(true)]), [Token::U16(42)]);
    }

    #[test]
    fn tokens_empty_eq() {
        assert_eq!(Tokens(vec![]), []);
    }

    #[test]
    fn tokens_multiple_eq() {
        assert_eq!(
            Tokens(vec![CanonicalToken::Bool(true), CanonicalToken::U8(42)]),
            [Token::Bool(true), Token::U8(42)]
        );
    }

    #[test]
    fn tokens_multiple_ne_values() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true), CanonicalToken::U8(42)]),
            [Token::Bool(false), Token::U8(42)]
        );
    }

    #[test]
    fn tokens_multiple_ne_shorter() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true), CanonicalToken::U8(42)]),
            [Token::Bool(true)]
        );
    }

    #[test]
    fn tokens_multiple_ne_longer() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true), CanonicalToken::U8(42)]),
            [Token::Bool(true), Token::U8(42), Token::U8(42)]
        );
    }

    #[test]
    fn tokens_unordered_eq_same_order() {
        assert_eq!(
            Tokens(vec![CanonicalToken::Bool(true), CanonicalToken::U8(42)]),
            [Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]])],
        );
    }

    #[test]
    fn tokens_unordered_eq_different_order() {
        assert_eq!(
            Tokens(vec![CanonicalToken::U8(42), CanonicalToken::Bool(true)]),
            [Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]])],
        );
    }

    #[test]
    fn tokens_unordered_eq_within_other_tokens() {
        assert_eq!(
            Tokens(vec![
                CanonicalToken::Char('a'),
                CanonicalToken::U8(42),
                CanonicalToken::Bool(true),
                CanonicalToken::I16(-42)
            ]),
            [
                Token::Char('a'),
                Token::Unordered(&[&[Token::Bool(true)], &[Token::U8(42)]]),
                Token::I16(-42)
            ],
        );
    }

    #[test]
    fn tokens_unordered_eq_multiple_tokens() {
        assert_eq!(
            Tokens(vec![
                CanonicalToken::U8(42),
                CanonicalToken::Bool(true),
                CanonicalToken::Char('a')
            ]),
            [Token::Unordered(&[
                &[Token::Bool(true), Token::Char('a')],
                &[Token::U8(42)]
            ])],
        );
    }

    #[test]
    fn tokens_unordered_ne_empty() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true)]),
            [Token::Unordered(&[])],
        );
    }

    #[test]
    fn tokens_unordered_ne_variant() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true)]),
            [Token::Unordered(&[&[Token::I8(42)]])],
        );
    }

    #[test]
    fn tokens_unordered_ne_value() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Bool(true)]),
            [Token::Unordered(&[&[Token::Bool(false)]])],
        );
    }

    #[test]
    fn tokens_unordered_nested() {
        assert_eq!(
            Tokens(vec![
                CanonicalToken::Unit,
                CanonicalToken::U8(4),
                CanonicalToken::U8(3),
                CanonicalToken::U8(1),
                CanonicalToken::U8(2),
                CanonicalToken::Bool(true)
            ]),
            [Token::Unordered(&[
                &[Token::Bool(true)],
                &[Token::Unordered(&[
                    &[Token::U8(1), Token::U8(2)],
                    &[Token::U8(3)],
                ])],
                &[Token::Unit, Token::U8(4)],
            ])]
        );
    }

    #[test]
    fn tokens_unordered_empty() {
        assert_eq!(
            Tokens(vec![CanonicalToken::Unit,]),
            [Token::Unordered(&[]), Token::Unit]
        );
    }

    #[test]
    fn tokens_unordered_empty_at_end() {
        assert_eq!(
            Tokens(vec![CanonicalToken::Unit,]),
            [Token::Unit, Token::Unordered(&[])]
        );
    }

    #[test]
    fn tokens_unordered_nonempty_at_end() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Unit,]),
            [Token::Unit, Token::Unordered(&[&[Token::Unit]])]
        );
    }

    #[test]
    fn tokens_end_within_unordered() {
        assert_ne!(
            Tokens(vec![CanonicalToken::Unit,]),
            [Token::Unordered(&[&[Token::Unit, ], &[Token::Unit]])]
        );
    }

    #[test]
    fn token_from_canonical_token_bool() {
        assert_matches!(Token::from(CanonicalToken::Bool(true)), Token::Bool(true))
    }

    #[test]
    fn token_from_canonical_token_i8() {
        assert_matches!(Token::from(CanonicalToken::I8(42)), Token::I8(42))
    }

    #[test]
    fn token_from_canonical_token_i16() {
        assert_matches!(Token::from(CanonicalToken::I16(42)), Token::I16(42))
    }

    #[test]
    fn token_from_canonical_token_i32() {
        assert_matches!(Token::from(CanonicalToken::I32(42)), Token::I32(42))
    }

    #[test]
    fn token_from_canonical_token_i64() {
        assert_matches!(Token::from(CanonicalToken::I64(42)), Token::I64(42))
    }

    #[test]
    fn token_from_canonical_token_i128() {
        assert_matches!(Token::from(CanonicalToken::I128(42)), Token::I128(42))
    }

    #[test]
    fn token_from_canonical_token_u8() {
        assert_matches!(Token::from(CanonicalToken::U8(42)), Token::U8(42))
    }

    #[test]
    fn token_from_canonical_token_u16() {
        assert_matches!(Token::from(CanonicalToken::U16(42)), Token::U16(42))
    }

    #[test]
    fn token_from_canonical_token_u32() {
        assert_matches!(Token::from(CanonicalToken::U32(42)), Token::U32(42))
    }

    #[test]
    fn token_from_canonical_token_u64() {
        assert_matches!(Token::from(CanonicalToken::U64(42)), Token::U64(42))
    }

    #[test]
    fn token_from_canonical_token_u128() {
        assert_matches!(Token::from(CanonicalToken::U128(42)), Token::U128(42))
    }

    #[test]
    fn token_from_canonical_token_f32() {
        assert_matches!(Token::from(CanonicalToken::F32(42.9)), Token::F32(_))
    }

    #[test]
    fn token_from_canonical_token_f64() {
        assert_matches!(Token::from(CanonicalToken::F64(42.9)), Token::F64(_))
    }

    #[test]
    fn token_from_canonical_token_char() {
        assert_matches!(Token::from(CanonicalToken::Char('a')), Token::Char('a'))
    }

    #[test]
    fn token_from_canonical_token_str() {
        assert_matches!(
            Token::from(CanonicalToken::Str("foo".to_owned())),
            Token::Str(_)
        )
    }

    #[test]
    fn token_from_canonical_token_bytes() {
        assert_matches!(
            Token::from(CanonicalToken::Bytes(b"foo".to_vec())),
            Token::Bytes(_)
        )
    }

    #[test]
    fn token_from_canonical_token_none() {
        assert_matches!(Token::from(CanonicalToken::None), Token::None)
    }

    #[test]
    fn token_from_canonical_token_some() {
        assert_matches!(Token::from(CanonicalToken::Some), Token::Some)
    }

    #[test]
    fn token_from_canonical_token_unit() {
        assert_matches!(Token::from(CanonicalToken::Unit), Token::Unit)
    }

    #[test]
    fn token_from_canonical_token_unit_struct() {
        assert_matches!(
            Token::from(CanonicalToken::UnitStruct { name: "foo" }),
            Token::UnitStruct { name: "foo" }
        )
    }

    #[test]
    fn token_from_canonical_token_unit_variant() {
        assert_matches!(
            Token::from(CanonicalToken::UnitVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar"
            }),
            Token::UnitVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar"
            }
        )
    }

    #[test]
    fn token_from_canonical_token_newtype_struct() {
        assert_matches!(
            Token::from(CanonicalToken::NewtypeStruct { name: "foo" }),
            Token::NewtypeStruct { name: "foo" }
        )
    }

    #[test]
    fn token_from_canonical_token_newtype_variant() {
        assert_matches!(
            Token::from(CanonicalToken::NewtypeVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar"
            }),
            Token::NewtypeVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar"
            }
        )
    }

    #[test]
    fn token_from_canonical_token_seq() {
        assert_matches!(
            Token::from(CanonicalToken::Seq { len: Some(42) }),
            Token::Seq { len: Some(42) }
        )
    }

    #[test]
    fn token_from_canonical_token_seq_end() {
        assert_matches!(Token::from(CanonicalToken::SeqEnd), Token::SeqEnd)
    }

    #[test]
    fn token_from_canonical_token_tuple() {
        assert_matches!(
            Token::from(CanonicalToken::Tuple { len: 42 }),
            Token::Tuple { len: 42 }
        )
    }

    #[test]
    fn token_from_canonical_token_tuple_end() {
        assert_matches!(Token::from(CanonicalToken::TupleEnd), Token::TupleEnd)
    }

    #[test]
    fn token_from_canonical_token_tuple_struct() {
        assert_matches!(
            Token::from(CanonicalToken::TupleStruct {
                name: "foo",
                len: 42
            }),
            Token::TupleStruct {
                name: "foo",
                len: 42
            }
        )
    }

    #[test]
    fn token_from_canonical_token_tuple_struct_end() {
        assert_matches!(
            Token::from(CanonicalToken::TupleStructEnd),
            Token::TupleStructEnd
        )
    }

    #[test]
    fn token_from_canonical_token_tuple_variant() {
        assert_matches!(
            Token::from(CanonicalToken::TupleVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar",
                len: 42
            }),
            Token::TupleVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar",
                len: 42
            }
        )
    }

    #[test]
    fn token_from_canonical_token_tuple_variant_end() {
        assert_matches!(
            Token::from(CanonicalToken::TupleVariantEnd),
            Token::TupleVariantEnd
        )
    }

    #[test]
    fn token_from_canonical_token_map() {
        assert_matches!(
            Token::from(CanonicalToken::Map { len: Some(42) }),
            Token::Map { len: Some(42) }
        )
    }

    #[test]
    fn token_from_canonical_token_map_end() {
        assert_matches!(Token::from(CanonicalToken::MapEnd), Token::MapEnd)
    }

    #[test]
    fn token_from_canonical_token_field() {
        assert_matches!(
            Token::from(CanonicalToken::Field("foo")),
            Token::Field("foo")
        )
    }

    #[test]
    fn token_from_canonical_token_skipped_field() {
        assert_matches!(
            Token::from(CanonicalToken::SkippedField("foo")),
            Token::SkippedField("foo")
        )
    }

    #[test]
    fn token_from_canonical_token_struct() {
        assert_matches!(
            Token::from(CanonicalToken::Struct {
                name: "foo",
                len: 42
            }),
            Token::Struct {
                name: "foo",
                len: 42
            }
        )
    }

    #[test]
    fn token_from_canonical_token_struct_end() {
        assert_matches!(Token::from(CanonicalToken::StructEnd), Token::StructEnd)
    }

    #[test]
    fn token_from_canonical_token_struct_variant() {
        assert_matches!(
            Token::from(CanonicalToken::StructVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar",
                len: 42
            }),
            Token::StructVariant {
                name: "foo",
                variant_index: 42,
                variant: "bar",
                len: 42
            }
        )
    }

    #[test]
    fn token_from_canonical_token_struct_variant_end() {
        assert_matches!(
            Token::from(CanonicalToken::StructVariantEnd),
            Token::StructVariantEnd
        )
    }

    #[test]
    fn unexpected_from_canonical_token_bool() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Bool(true)),
            Unexpected::Bool(true)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_i8() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::I8(42)),
            Unexpected::Signed(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_i16() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::I16(42)),
            Unexpected::Signed(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_i32() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::I32(42)),
            Unexpected::Signed(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_i64() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::I64(42)),
            Unexpected::Signed(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_i128() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::I128(42)),
            Unexpected::Other("i128")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_u8() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::U8(42)),
            Unexpected::Unsigned(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_u16() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::U16(42)),
            Unexpected::Unsigned(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_u32() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::U32(42)),
            Unexpected::Unsigned(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_u64() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::U64(42)),
            Unexpected::Unsigned(42)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_u128() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::U128(42)),
            Unexpected::Other("u128")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_f32() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::F32(42.)),
            Unexpected::Float(42.)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_f64() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::F64(42.)),
            Unexpected::Float(42.)
        )
    }

    #[test]
    fn unexpected_from_canonical_token_char() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Char('a')),
            Unexpected::Char('a')
        )
    }

    #[test]
    fn unexpected_from_canonical_token_str() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Str("foo".to_owned())),
            Unexpected::Str("foo")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_bytes() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Bytes(b"foo".to_vec())),
            Unexpected::Bytes(b"foo")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_some() {
        assert_eq!(Unexpected::from(&CanonicalToken::Some), Unexpected::Option)
    }

    #[test]
    fn unexpected_from_canonical_token_none() {
        assert_eq!(Unexpected::from(&CanonicalToken::None), Unexpected::Option)
    }

    #[test]
    fn unexpected_from_canonical_token_unit() {
        assert_eq!(Unexpected::from(&CanonicalToken::Unit), Unexpected::Unit)
    }

    #[test]
    fn unexpected_from_canonical_token_unit_struct() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::UnitStruct { name: "foo" }),
            Unexpected::Unit
        )
    }

    #[test]
    fn unexpected_from_canonical_token_unit_variant() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::UnitVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar"
            }),
            Unexpected::UnitVariant
        )
    }

    #[test]
    fn unexpected_from_canonical_token_newtype_struct() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::NewtypeStruct { name: "foo" }),
            Unexpected::NewtypeStruct
        )
    }

    #[test]
    fn unexpected_from_canonical_token_newtype_variant() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::NewtypeVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar"
            }),
            Unexpected::NewtypeVariant
        )
    }

    #[test]
    fn unexpected_from_canonical_token_seq() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Seq { len: None }),
            Unexpected::Seq
        )
    }

    #[test]
    fn unexpected_from_canonical_token_tuple() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Tuple { len: 0 }),
            Unexpected::Seq
        )
    }

    #[test]
    fn unexpected_from_canonical_token_seq_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::SeqEnd),
            Unexpected::Other("SeqEnd")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_tuple_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::TupleEnd),
            Unexpected::Other("TupleEnd")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_tuple_struct() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::TupleStruct {
                name: "foo",
                len: 0
            }),
            Unexpected::Other("TupleStruct")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_tuple_struct_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::TupleStructEnd),
            Unexpected::Other("TupleStructEnd")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_tuple_variant() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::TupleVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar",
                len: 0
            }),
            Unexpected::TupleVariant
        )
    }

    #[test]
    fn unexpected_from_canonical_token_tuple_variant_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::TupleVariantEnd),
            Unexpected::Other("TupleVariantEnd")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_map() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Map { len: None }),
            Unexpected::Map
        )
    }

    #[test]
    fn unexpected_from_canonical_token_map_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::MapEnd),
            Unexpected::Other("MapEnd")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_field() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Field("foo")),
            Unexpected::Other("Field")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_skipped_field() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::SkippedField("foo")),
            Unexpected::Other("SkippedField")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_struct() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::Struct {
                name: "foo",
                len: 0
            }),
            Unexpected::Other("Struct")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_struct_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::StructEnd),
            Unexpected::Other("StructEnd")
        )
    }

    #[test]
    fn unexpected_from_canonical_token_struct_variant() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::StructVariant {
                name: "foo",
                variant_index: 0,
                variant: "bar",
                len: 0
            }),
            Unexpected::StructVariant
        )
    }

    #[test]
    fn unexpected_from_canonical_token_struct_variant_end() {
        assert_eq!(
            Unexpected::from(&CanonicalToken::StructVariantEnd),
            Unexpected::Other("StructVariantEnd")
        )
    }

    #[test]
    fn owning_iter_empty() {
        let mut iter = OwningIter::new(Tokens(Vec::new()));

        assert_none!(iter.next());
    }

    #[test]
    fn owning_iter_one_token() {
        let mut iter = OwningIter::new(Tokens(vec![CanonicalToken::Bool(true)]));

        assert_some_eq!(iter.next(), &CanonicalToken::Bool(true));
        assert_none!(iter.next());
    }

    #[test]
    fn owning_iter_multiple_tokens() {
        let mut iter = OwningIter::new(Tokens(vec![
            CanonicalToken::Bool(true),
            CanonicalToken::U64(42),
            CanonicalToken::Str("foo".to_owned()),
        ]));

        assert_some_eq!(iter.next(), &CanonicalToken::Bool(true));
        assert_some_eq!(iter.next(), &CanonicalToken::U64(42));
        assert_some_eq!(iter.next(), &CanonicalToken::Str("foo".to_owned()));
        assert_none!(iter.next());
    }

    #[test]
    fn owning_iter_empty_debug() {
        let iter = OwningIter::new(Tokens(Vec::new()));

        assert_eq!(format!("{:?}", iter), "OwningIter([])")
    }

    #[test]
    fn owning_iter_uniterated_debug() {
        let iter = OwningIter::new(Tokens(vec![
            CanonicalToken::Bool(true),
            CanonicalToken::U64(42),
            CanonicalToken::Str("foo".to_owned()),
        ]));

        assert_eq!(
            format!("{:?}", iter),
            "OwningIter([Bool(true), U64(42), Str(\"foo\")])"
        )
    }

    #[test]
    fn owning_iter_partially_iterated_debug() {
        let mut iter = OwningIter::new(Tokens(vec![
            CanonicalToken::Bool(true),
            CanonicalToken::U64(42),
            CanonicalToken::Str("foo".to_owned()),
        ]));

        assert_some!(iter.next());

        assert_eq!(format!("{:?}", iter), "OwningIter([U64(42), Str(\"foo\")])")
    }

    #[test]
    fn owning_iter_fully_iterated_debug() {
        let mut iter = OwningIter::new(Tokens(vec![
            CanonicalToken::Bool(true),
            CanonicalToken::U64(42),
            CanonicalToken::Str("foo".to_owned()),
        ]));

        assert_some!(iter.next());
        assert_some!(iter.next());
        assert_some!(iter.next());

        assert_eq!(format!("{:?}", iter), "OwningIter([])")
    }
}
