//! Testing serialization implementations.
//!
//! This module provides a [`Serializer`] struct for testing serialization. Construction of this
//! struct uses the builder pattern through the [`Builder`] struct, allowing configuration of the
//! behavior of the `Serializer`.
//!
//! # Example
//!
//! ``` rust
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

use crate::{
    Token,
    Tokens,
};
use alloc::{
    borrow::ToOwned,
    string::{
        String,
        ToString,
    },
    vec,
};
use core::{
    fmt,
    fmt::Display,
};
use serde::{
    ser,
    ser::{
        SerializeMap,
        SerializeSeq,
        SerializeStruct,
        SerializeStructVariant,
        SerializeTuple,
        SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serialize,
};

/// Serializer for testing [`Serialize`] implementations.
///
/// This serializer outputs [`Tokens`] representing the serialized value. The `Tokens` can be
/// compared against expected `Tokens` to ensure the serialization is correct.
///
/// # Configuration
/// The following options can be configured on the [`Builder`]:
///
/// - [`is_human_readable()`]: Determines whether the serializer will serialize values in a
/// readable format or a compact format. Useful for complicated structs wishing to provide
/// different outputs depending on the readability of the serialization type.
///
/// # Example
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
/// [`is_human_readable()`]: Builder::is_human_readable()
/// [`Serialize`]: serde::Serialize
#[derive(Debug)]
pub struct Serializer {
    is_human_readable: bool,
}

impl<'a> ser::Serializer for &'a Serializer {
    type Ok = Tokens;
    type Error = Error;

    type SerializeSeq = CompoundSerializer<'a>;
    type SerializeTuple = CompoundSerializer<'a>;
    type SerializeTupleStruct = CompoundSerializer<'a>;
    type SerializeTupleVariant = CompoundSerializer<'a>;
    type SerializeMap = CompoundSerializer<'a>;
    type SerializeStruct = CompoundSerializer<'a>;
    type SerializeStructVariant = CompoundSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::Bool(v)]))
    }

    fn serialize_i8(self, v: i8) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::I8(v)]))
    }

    fn serialize_i16(self, v: i16) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::I16(v)]))
    }

    fn serialize_i32(self, v: i32) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::I32(v)]))
    }

    fn serialize_i64(self, v: i64) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::I64(v)]))
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::I128(v)]))
    }

    fn serialize_u8(self, v: u8) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::U8(v)]))
    }

    fn serialize_u16(self, v: u16) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::U16(v)]))
    }

    fn serialize_u32(self, v: u32) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::U32(v)]))
    }

    fn serialize_u64(self, v: u64) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::U64(v)]))
    }

    #[cfg(has_i128)]
    fn serialize_u128(self, v: u128) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::U128(v)]))
    }

    fn serialize_f32(self, v: f32) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::F32(v)]))
    }

    fn serialize_f64(self, v: f64) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::F64(v)]))
    }

    fn serialize_char(self, v: char) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::Char(v)]))
    }

    fn serialize_str(self, v: &str) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::Str(v.to_owned())]))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::Bytes(v.to_owned())]))
    }

    fn serialize_none(self) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::None]))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Tokens, Error>
    where
        T: Serialize + ?Sized,
    {
        let mut tokens = Tokens(vec![Token::Some]);
        tokens.0.extend(value.serialize(self)?.0);
        Ok(tokens)
    }

    fn serialize_unit(self) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::Unit]))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::UnitStruct { name }]))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Tokens, Error> {
        Ok(Tokens(vec![Token::UnitVariant {
            name,
            variant_index,
            variant,
        }]))
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Tokens, Error>
    where
        T: Serialize + ?Sized,
    {
        let mut tokens = Tokens(vec![Token::NewtypeStruct { name }]);
        tokens.0.extend(value.serialize(self)?.0);
        Ok(tokens)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Tokens, Error>
    where
        T: Serialize + ?Sized,
    {
        let mut tokens = Tokens(vec![Token::NewtypeVariant {
            name,
            variant_index,
            variant,
        }]);
        tokens.0.extend(value.serialize(self)?.0);
        Ok(tokens)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::Seq { len }]),

            serializer: self,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::Tuple { len }]),

            serializer: self,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::TupleStruct { name, len }]),

            serializer: self,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::TupleVariant {
                name,
                variant_index,
                variant,
                len,
            }]),

            serializer: self,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::Map { len }]),

            serializer: self,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::Struct { name, len }]),

            serializer: self,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![Token::StructVariant {
                name,
                variant_index,
                variant,
                len,
            }]),

            serializer: self,
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Tokens, Error>
    where
        T: Display + ?Sized,
    {
        Ok(Tokens(vec![Token::Str(value.to_string())]))
    }

    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

impl Serializer {
    /// Returns a [`Builder`] for a [`Serializer`].
    ///
    /// # Example
    /// ``` rust
    /// use serde_assert::Serializer;
    ///
    /// let serializer = Serializer::builder().is_human_readable(false).build();
    /// ```
    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }
}

/// A builder for a [`Serializer`].
///
/// Construction of a `Serializer` follows the builder pattern. Configuration options can be set on
/// the `Builder`, and then the actual `Serializer` is constructed by calling [`build()`].
///
/// # Example
/// ``` rust
/// use serde_assert::Serializer;
///
/// let serializer = Serializer::builder().is_human_readable(false).build();
/// ```
///
/// [`build()`]: Builder::build()
#[derive(Debug, Default)]
pub struct Builder {
    is_human_readable: Option<bool>,
}

impl Builder {
    /// Determines whether the serializer will serialize values in a readable format or a compact
    /// format.
    ///
    /// Useful for complicated structs wishing to provide different outputs depending on
    /// the readability of the serialization type.
    ///
    /// If not set, the default value is `true`.
    ///
    /// # Example
    /// ``` rust
    /// use serde_assert::Serializer;
    ///
    /// let serializer = Serializer::builder().is_human_readable(false).build();
    /// ```
    pub fn is_human_readable(&mut self, is_human_readable: bool) -> &mut Self {
        self.is_human_readable = Some(is_human_readable);
        self
    }

    /// Build a new [`Serializer`] using this `Builder`.
    ///
    /// Constructs a new `Serializer` using the configuration options set on this `Builder`.
    ///
    /// # Example
    /// ``` rust
    /// use serde_assert::Serializer;
    ///
    /// let serializer = Serializer::builder().is_human_readable(false).build();
    /// ```
    pub fn build(&mut self) -> Serializer {
        Serializer {
            is_human_readable: self.is_human_readable.unwrap_or(true),
        }
    }
}

/// Serializer for serializing compound types.
///
/// This type implements [`SerializeSeq`], [`SerializeTuple`], [`SerializeTupleStruct`],
/// [`SerializeTupleVariant`], [`SerializeMap`], [`SerializeStruct`], and
/// [`SerializeStructVariant`], and is used by [`Serializer`] for serialization of each of those
/// compound data types.
///
/// Users normally will not need to interact with this type directly. It is primarily used by
/// [`Serialize`] implementations through the various traits it implements.
#[derive(Debug)]
pub struct CompoundSerializer<'a> {
    tokens: Tokens,

    serializer: &'a Serializer,
}

impl SerializeSeq for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::SeqEnd);
        Ok(self.tokens)
    }
}

impl SerializeTuple for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::TupleEnd);
        Ok(self.tokens)
    }
}

impl SerializeTupleStruct for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::TupleStructEnd);
        Ok(self.tokens)
    }
}

impl SerializeTupleVariant for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::TupleVariantEnd);
        Ok(self.tokens)
    }
}

impl SerializeMap for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_key<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::MapEnd);
        Ok(self.tokens)
    }
}

impl SerializeStruct for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.push(Token::Field(key));
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.tokens.0.push(Token::SkippedField(key));
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::StructEnd);
        Ok(self.tokens)
    }
}

impl SerializeStructVariant for CompoundSerializer<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        self.tokens.0.push(Token::Field(key));
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.tokens.0.push(Token::SkippedField(key));
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(Token::StructVariantEnd);
        Ok(self.tokens)
    }
}

/// An error encountered during serialization.
///
/// # Example
/// ```rust
/// use serde::ser::Error as _;
/// use serde_assert::ser::Error;
///
/// assert_eq!(format!("{}", Error::custom("foo")), "foo");
/// ```
#[derive(Debug)]
pub struct Error(pub String);

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl ser::StdError for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Error,
        Serializer,
    };
    use crate::{
        Token,
        Tokens,
    };
    use alloc::{
        borrow::ToOwned,
        format,
        string::String,
        vec,
    };
    use claims::assert_ok_eq;
    use hashbrown::{
        HashMap,
        HashSet,
    };
    use serde::ser::{
        Error as _,
        Serialize,
        Serializer as _,
    };
    use serde_bytes::Bytes;
    use serde_derive::Serialize;

    #[test]
    fn serialize_bool() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(true.serialize(&serializer), Tokens(vec![Token::Bool(true)]));
    }

    #[test]
    fn serialize_i8() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i8.serialize(&serializer), Tokens(vec![Token::I8(42)]));
    }

    #[test]
    fn serialize_i16() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i16.serialize(&serializer), Tokens(vec![Token::I16(42)]));
    }

    #[test]
    fn serialize_i32() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i32.serialize(&serializer), Tokens(vec![Token::I32(42)]));
    }

    #[test]
    fn serialize_i64() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i64.serialize(&serializer), Tokens(vec![Token::I64(42)]));
    }

    #[cfg(has_i128)]
    #[test]
    fn serialize_i128() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i128.serialize(&serializer), Tokens(vec![Token::I128(42)]));
    }

    #[test]
    fn serialize_u8() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u8.serialize(&serializer), Tokens(vec![Token::U8(42)]));
    }

    #[test]
    fn serialize_u16() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u16.serialize(&serializer), Tokens(vec![Token::U16(42)]));
    }

    #[test]
    fn serialize_u32() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u32.serialize(&serializer), Tokens(vec![Token::U32(42)]));
    }

    #[test]
    fn serialize_u64() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u64.serialize(&serializer), Tokens(vec![Token::U64(42)]));
    }

    #[cfg(has_i128)]
    #[test]
    fn serialize_u128() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u128.serialize(&serializer), Tokens(vec![Token::U128(42)]));
    }

    #[test]
    fn serialize_f32() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42f32.serialize(&serializer), Tokens(vec![Token::F32(42.)]));
    }

    #[test]
    fn serialize_f64() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42f64.serialize(&serializer), Tokens(vec![Token::F64(42.)]));
    }

    #[test]
    fn serialize_char() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!('a'.serialize(&serializer), Tokens(vec![Token::Char('a')]));
    }

    #[test]
    fn serialize_str() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            "a".serialize(&serializer),
            Tokens(vec![Token::Str("a".to_owned())])
        );
    }

    #[test]
    fn serialize_bytes() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Bytes::new(b"a").serialize(&serializer),
            Tokens(vec![Token::Bytes(b"a".to_vec())])
        );
    }

    #[test]
    fn serialize_none() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Option::<()>::None.serialize(&serializer),
            Tokens(vec![Token::None])
        );
    }

    #[test]
    fn serialize_some() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Some(true).serialize(&serializer),
            Tokens(vec![Token::Some, Token::Bool(true)])
        );
    }

    #[test]
    fn serialize_unit() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(().serialize(&serializer), Tokens(vec![Token::Unit]));
    }

    #[test]
    fn serialize_unit_struct() {
        #[derive(Serialize)]
        struct Unit;

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Unit.serialize(&serializer),
            Tokens(vec![Token::UnitStruct { name: "Unit" }])
        );
    }

    #[test]
    fn serialize_unit_variant() {
        #[derive(Serialize)]
        enum Unit {
            Variant,
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Unit::Variant.serialize(&serializer),
            Tokens(vec![Token::UnitVariant {
                name: "Unit",
                variant_index: 0,
                variant: "Variant"
            }])
        );
    }

    #[test]
    fn serialize_newtype_struct() {
        #[derive(Serialize)]
        struct Newtype(bool);

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Newtype(false).serialize(&serializer),
            Tokens(vec![
                Token::NewtypeStruct { name: "Newtype" },
                Token::Bool(false)
            ])
        );
    }

    #[test]
    fn serialize_newtype_variant() {
        #[derive(Serialize)]
        enum Newtype {
            Variant(bool),
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Newtype::Variant(false).serialize(&serializer),
            Tokens(vec![
                Token::NewtypeVariant {
                    name: "Newtype",
                    variant_index: 0,
                    variant: "Variant"
                },
                Token::Bool(false)
            ])
        );
    }

    #[test]
    fn serialize_seq() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            vec![1i8, 2i8, 3i8].serialize(&serializer),
            Tokens(vec![
                Token::Seq { len: Some(3) },
                Token::I8(1),
                Token::I8(2),
                Token::I8(3),
                Token::SeqEnd
            ]),
        );
    }

    #[test]
    fn serialize_seq_unordered() {
        let serializer = Serializer::builder().build();

        let mut set = HashSet::new();
        set.insert('a');
        set.insert('b');
        set.insert('c');

        assert_ok_eq!(
            set.serialize(&serializer),
            Tokens(vec![
                Token::Seq { len: Some(3) },
                Token::Unordered(&[
                    &[Token::Char('a')],
                    &[Token::Char('b')],
                    &[Token::Char('c')],
                ]),
                Token::SeqEnd,
            ])
        );
    }

    #[test]
    fn serialize_tuple() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            (1i8, 2i16, 3i32).serialize(&serializer),
            Tokens(vec![
                Token::Tuple { len: 3 },
                Token::I8(1),
                Token::I16(2),
                Token::I32(3),
                Token::TupleEnd
            ]),
        );
    }

    #[test]
    fn serialize_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(i8, i16, i32);

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            TupleStruct(1i8, 2i16, 3i32).serialize(&serializer),
            Tokens(vec![
                Token::TupleStruct {
                    name: "TupleStruct",
                    len: 3
                },
                Token::I8(1),
                Token::I16(2),
                Token::I32(3),
                Token::TupleStructEnd
            ]),
        );
    }

    #[test]
    fn serialize_tuple_variant() {
        #[derive(Serialize)]
        enum Tuple {
            Variant(i8, i16, i32),
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Tuple::Variant(1i8, 2i16, 3i32).serialize(&serializer),
            Tokens(vec![
                Token::TupleVariant {
                    name: "Tuple",
                    variant_index: 0,
                    variant: "Variant",
                    len: 3
                },
                Token::I8(1),
                Token::I16(2),
                Token::I32(3),
                Token::TupleVariantEnd
            ]),
        );
    }

    #[test]
    fn serialize_map() {
        let serializer = Serializer::builder().build();

        let mut map = HashMap::new();
        map.insert(1i8, 'a');
        map.insert(2i8, 'b');
        map.insert(3i8, 'c');

        assert_ok_eq!(
            map.serialize(&serializer),
            Tokens(vec![
                Token::Map { len: Some(3) },
                Token::Unordered(&[
                    &[Token::I8(1), Token::Char('a')],
                    &[Token::I8(2), Token::Char('b')],
                    &[Token::I8(3), Token::Char('c')],
                ]),
                Token::MapEnd,
            ])
        );
    }

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct Struct {
            a: bool,
            b: u16,
            c: String,
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Struct {
                a: true,
                b: 42,
                c: "foo".to_owned(),
            }
            .serialize(&serializer),
            Tokens(vec![
                Token::Struct {
                    name: "Struct",
                    len: 3,
                },
                Token::Field("a"),
                Token::Bool(true),
                Token::Field("b"),
                Token::U16(42),
                Token::Field("c"),
                Token::Str("foo".to_owned()),
                Token::StructEnd,
            ])
        );
    }

    #[test]
    fn serialize_struct_skipped_field() {
        fn skip<T>(_: &T) -> bool {
            true
        }

        #[derive(Serialize)]
        struct Struct {
            a: bool,
            #[serde(skip_serializing_if = "skip")]
            b: u16,
            c: String,
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Struct {
                a: true,
                b: 42,
                c: "foo".to_owned(),
            }
            .serialize(&serializer),
            Tokens(vec![
                Token::Struct {
                    name: "Struct",
                    len: 2,
                },
                Token::Field("a"),
                Token::Bool(true),
                Token::SkippedField("b"),
                Token::Field("c"),
                Token::Str("foo".to_owned()),
                Token::StructEnd,
            ])
        );
    }

    #[test]
    fn serialize_struct_variant() {
        #[derive(Serialize)]
        enum Struct {
            Variant { a: bool, b: u16, c: String },
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Struct::Variant {
                a: true,
                b: 42,
                c: "foo".to_owned(),
            }
            .serialize(&serializer),
            Tokens(vec![
                Token::StructVariant {
                    name: "Struct",
                    variant_index: 0,
                    variant: "Variant",
                    len: 3,
                },
                Token::Field("a"),
                Token::Bool(true),
                Token::Field("b"),
                Token::U16(42),
                Token::Field("c"),
                Token::Str("foo".to_owned()),
                Token::StructVariantEnd,
            ])
        );
    }

    #[test]
    fn serialize_struct_variant_skipped_field() {
        fn skip<T>(_: &T) -> bool {
            true
        }

        #[derive(Serialize)]
        enum Struct {
            Variant {
                a: bool,
                #[serde(skip_serializing_if = "skip")]
                b: u16,
                c: String,
            },
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Struct::Variant {
                a: true,
                b: 42,
                c: "foo".to_owned(),
            }
            .serialize(&serializer),
            Tokens(vec![
                Token::StructVariant {
                    name: "Struct",
                    variant_index: 0,
                    variant: "Variant",
                    len: 2,
                },
                Token::Field("a"),
                Token::Bool(true),
                Token::SkippedField("b"),
                Token::Field("c"),
                Token::Str("foo".to_owned()),
                Token::StructVariantEnd,
            ])
        );
    }

    #[test]
    fn collect_str() {
        struct CollectedString(String);

        impl Serialize for CollectedString {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.collect_str(&self.0)
            }
        }

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            CollectedString("foo".to_owned()).serialize(&serializer),
            Tokens(vec![Token::Str("foo".to_owned())])
        );
    }

    #[test]
    fn is_human_readable_default() {
        let serializer = Serializer::builder().build();

        assert!((&serializer).is_human_readable());
    }

    #[test]
    fn is_human_readable_false() {
        let serializer = Serializer::builder().is_human_readable(false).build();

        assert!(!(&serializer).is_human_readable());
    }

    #[test]
    fn is_human_readable_true() {
        let serializer = Serializer::builder().is_human_readable(true).build();

        assert!((&serializer).is_human_readable());
    }

    #[test]
    fn custom_error() {
        let error = Error::custom("foo");

        assert_eq!(error.0, "foo");
    }

    #[test]
    fn display_error() {
        let formatted = format!("{}", Error::custom("foo"));

        assert_eq!(formatted, "foo");
    }
}
