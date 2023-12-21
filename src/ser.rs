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
//! };
//!
//! let serializer = Serializer::builder().build();
//!
//! assert_ok_eq!(true.serialize(&serializer), [Token::Bool(true)]);
//! ```

use crate::token::{
    CanonicalToken,
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
        SerializeStructVariant,
        SerializeTuple,
        SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serialize,
};

/// Configuration for serializing `struct`s.
///
/// Can be passed to a [`Builder`] to determine how `struct`s should be serialized by the
/// [`Serializer`].
///
/// # Example
/// ``` rust
/// use claims::assert_ok_eq;
/// use serde::Serialize;
/// use serde_assert::{
///     ser::SerializeStructAs,
///     Serializer,
///     Token,
/// };
/// # use serde_derive::Serialize;
///
/// #[derive(Serialize)]
/// struct Struct {
///     foo: bool,
///     bar: u32,
/// }
///
/// let some_struct = Struct {
///     foo: false,
///     bar: 42,
/// };
/// let serializer = Serializer::builder()
///     .serialize_struct_as(SerializeStructAs::Seq)
///     .build();
///
/// assert_ok_eq!(
///     some_struct.serialize(&serializer),
///     [
///         Token::Seq { len: Some(2) },
///         Token::Bool(false),
///         Token::U32(42),
///         Token::SeqEnd,
///     ]
/// );
/// ```
#[derive(Clone, Copy, Debug)]
pub enum SerializeStructAs {
    /// Serialize structs using [`Token::Struct`].
    Struct,
    /// Serialize structs using [`Token::Seq`].
    ///
    /// This type of serialization is often done by compact serialization formats. Using this
    /// setting simulates those serializers.
    Seq,
}

/// Serializer for testing [`Serialize`] implementations.
///
/// This serializer outputs [`Tokens`] representing the serialized value. The `Tokens` can be
/// compared against an expected sequence of [`Token`]s to ensure the serialization is correct.
///
/// # Configuration
/// The following options can be configured on the [`Builder`]:
///
/// - [`is_human_readable()`]: Determines whether the serializer will serialize values in a
/// readable format or a compact format. Useful for complicated structs wishing to provide
/// different outputs depending on the readability of the serialization type.
/// - [`serialize_struct_as()`]: Specifies how the serializer should serialize structs. Compact
/// formats often serialize structs as sequences. By enabling this setting, tokens can be produced
/// in this format, and can then be deserialized to ensure structs deserialized as sequences are
/// deserialized correctly.
///
/// # Example
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
/// [`is_human_readable()`]: Builder::is_human_readable()
/// [`serialize_struct_as()`]: Builder::serialize_struct_as()
/// [`Serialize`]: serde::Serialize
#[derive(Debug)]
pub struct Serializer {
    is_human_readable: bool,
    serialize_struct_as: SerializeStructAs,
}

impl<'a> ser::Serializer for &'a Serializer {
    type Ok = Tokens;
    type Error = Error;

    type SerializeSeq = CompoundSerializer<'a>;
    type SerializeTuple = CompoundSerializer<'a>;
    type SerializeTupleStruct = CompoundSerializer<'a>;
    type SerializeTupleVariant = CompoundSerializer<'a>;
    type SerializeMap = CompoundSerializer<'a>;
    type SerializeStruct = SerializeStruct<'a>;
    type SerializeStructVariant = CompoundSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::Bool(v)]))
    }

    fn serialize_i8(self, v: i8) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::I8(v)]))
    }

    fn serialize_i16(self, v: i16) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::I16(v)]))
    }

    fn serialize_i32(self, v: i32) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::I32(v)]))
    }

    fn serialize_i64(self, v: i64) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::I64(v)]))
    }

    fn serialize_i128(self, v: i128) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::I128(v)]))
    }

    fn serialize_u8(self, v: u8) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::U8(v)]))
    }

    fn serialize_u16(self, v: u16) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::U16(v)]))
    }

    fn serialize_u32(self, v: u32) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::U32(v)]))
    }

    fn serialize_u64(self, v: u64) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::U64(v)]))
    }

    fn serialize_u128(self, v: u128) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::U128(v)]))
    }

    fn serialize_f32(self, v: f32) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::F32(v)]))
    }

    fn serialize_f64(self, v: f64) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::F64(v)]))
    }

    fn serialize_char(self, v: char) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::Char(v)]))
    }

    fn serialize_str(self, v: &str) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::Str(v.to_owned())]))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::Bytes(v.to_owned())]))
    }

    fn serialize_none(self) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::None]))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Tokens, Error>
    where
        T: Serialize + ?Sized,
    {
        let mut tokens = Tokens(vec![CanonicalToken::Some]);
        tokens.0.extend(value.serialize(self)?.0);
        Ok(tokens)
    }

    fn serialize_unit(self) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::Unit]))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::UnitStruct { name }]))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Tokens, Error> {
        Ok(Tokens(vec![CanonicalToken::UnitVariant {
            name,
            variant_index,
            variant,
        }]))
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Tokens, Error>
    where
        T: Serialize + ?Sized,
    {
        let mut tokens = Tokens(vec![CanonicalToken::NewtypeStruct { name }]);
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
        let mut tokens = Tokens(vec![CanonicalToken::NewtypeVariant {
            name,
            variant_index,
            variant,
        }]);
        tokens.0.extend(value.serialize(self)?.0);
        Ok(tokens)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![CanonicalToken::Seq { len }]),

            serializer: self,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![CanonicalToken::Tuple { len }]),

            serializer: self,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![CanonicalToken::TupleStruct { name, len }]),

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
            tokens: Tokens(vec![CanonicalToken::TupleVariant {
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
            tokens: Tokens(vec![CanonicalToken::Map { len }]),

            serializer: self,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<SerializeStruct<'a>, Error> {
        match self.serialize_struct_as {
            SerializeStructAs::Struct => Ok(SerializeStruct {
                tokens: Tokens(vec![CanonicalToken::Struct { name, len }]),

                serializer: self,

                serialize_struct_as: self.serialize_struct_as,
            }),
            SerializeStructAs::Seq => Ok(SerializeStruct {
                tokens: Tokens(vec![CanonicalToken::Seq { len: Some(len) }]),

                serializer: self,

                serialize_struct_as: self.serialize_struct_as,
            }),
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: Tokens(vec![CanonicalToken::StructVariant {
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
        Ok(Tokens(vec![CanonicalToken::Str(value.to_string())]))
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
#[derive(Debug)]
pub struct Builder {
    is_human_readable: bool,
    serialize_struct_as: SerializeStructAs,
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
        self.is_human_readable = is_human_readable;
        self
    }

    /// Specifies how the serializer should serialize structs.
    ///
    /// Compact formats often serialize structs as sequences. By enabling this setting, tokens can
    /// be produced in this format, and can then be deserialized to ensure structs deserialized as
    /// sequences are deserialized correctly.
    ///
    /// If not set, the default value is [`SerializeStructAs::Struct`].
    ///
    /// # Example
    /// ``` rust
    /// use claims::assert_ok_eq;
    /// use serde::Serialize;
    /// use serde_assert::{
    ///     ser::SerializeStructAs,
    ///     Serializer,
    ///     Token,
    /// };
    /// # use serde_derive::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Struct {
    ///     foo: bool,
    ///     bar: u32,
    /// }
    ///
    /// let some_struct = Struct {
    ///     foo: false,
    ///     bar: 42,
    /// };
    /// let serializer = Serializer::builder()
    ///     .serialize_struct_as(SerializeStructAs::Seq)
    ///     .build();
    ///
    /// assert_ok_eq!(
    ///     some_struct.serialize(&serializer),
    ///     [
    ///         Token::Seq { len: Some(2) },
    ///         Token::Bool(false),
    ///         Token::U32(42),
    ///         Token::SeqEnd,
    ///     ]
    /// );
    /// ```
    pub fn serialize_struct_as(&mut self, serialize_struct_as: SerializeStructAs) -> &mut Self {
        self.serialize_struct_as = serialize_struct_as;
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
            is_human_readable: self.is_human_readable,
            serialize_struct_as: self.serialize_struct_as,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            is_human_readable: true,
            serialize_struct_as: SerializeStructAs::Struct,
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
        self.tokens.0.push(CanonicalToken::SeqEnd);
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
        self.tokens.0.push(CanonicalToken::TupleEnd);
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
        self.tokens.0.push(CanonicalToken::TupleStructEnd);
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
        self.tokens.0.push(CanonicalToken::TupleVariantEnd);
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
        self.tokens.0.push(CanonicalToken::MapEnd);
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
        self.tokens.0.push(CanonicalToken::Field(key));
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.tokens.0.push(CanonicalToken::SkippedField(key));
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(CanonicalToken::StructVariantEnd);
        Ok(self.tokens)
    }
}

/// Serializer for serializing `struct`s.
///
/// Users normally will not need to interact with this type directly. It is primarily used by
/// [`Serialize`] implementations through the [`serde::ser::SerializeStruct`] trait it implements.
pub struct SerializeStruct<'a> {
    tokens: Tokens,

    serializer: &'a Serializer,

    serialize_struct_as: SerializeStructAs,
}

impl ser::SerializeStruct for SerializeStruct<'_> {
    type Ok = Tokens;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize + ?Sized,
    {
        if matches!(self.serialize_struct_as, SerializeStructAs::Struct) {
            self.tokens.0.push(CanonicalToken::Field(key));
        }
        self.tokens.0.extend(value.serialize(self.serializer)?.0);
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.tokens.0.push(CanonicalToken::SkippedField(key));
        Ok(())
    }

    fn end(mut self) -> Result<Tokens, Error> {
        self.tokens.0.push(match self.serialize_struct_as {
            SerializeStructAs::Struct => CanonicalToken::StructEnd,
            SerializeStructAs::Seq => CanonicalToken::SeqEnd,
        });
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
#[derive(Debug, Eq, PartialEq)]
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
        SerializeStructAs,
        Serializer,
    };
    use crate::Token;
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

        assert_ok_eq!(true.serialize(&serializer), [Token::Bool(true)]);
    }

    #[test]
    fn serialize_i8() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i8.serialize(&serializer), [Token::I8(42)]);
    }

    #[test]
    fn serialize_i16() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i16.serialize(&serializer), [Token::I16(42)]);
    }

    #[test]
    fn serialize_i32() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i32.serialize(&serializer), [Token::I32(42)]);
    }

    #[test]
    fn serialize_i64() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i64.serialize(&serializer), [Token::I64(42)]);
    }

    #[test]
    fn serialize_i128() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42i128.serialize(&serializer), [Token::I128(42)]);
    }

    #[test]
    fn serialize_u8() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u8.serialize(&serializer), [Token::U8(42)]);
    }

    #[test]
    fn serialize_u16() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u16.serialize(&serializer), [Token::U16(42)]);
    }

    #[test]
    fn serialize_u32() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u32.serialize(&serializer), [Token::U32(42)]);
    }

    #[test]
    fn serialize_u64() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u64.serialize(&serializer), [Token::U64(42)]);
    }

    #[test]
    fn serialize_u128() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42u128.serialize(&serializer), [Token::U128(42)]);
    }

    #[test]
    fn serialize_f32() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42f32.serialize(&serializer), [Token::F32(42.)]);
    }

    #[test]
    fn serialize_f64() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(42f64.serialize(&serializer), [Token::F64(42.)]);
    }

    #[test]
    fn serialize_char() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!('a'.serialize(&serializer), [Token::Char('a')]);
    }

    #[test]
    fn serialize_str() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!("a".serialize(&serializer), [Token::Str("a".to_owned())]);
    }

    #[test]
    fn serialize_bytes() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Bytes::new(b"a").serialize(&serializer),
            [Token::Bytes(b"a".to_vec())]
        );
    }

    #[test]
    fn serialize_none() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(Option::<()>::None.serialize(&serializer), [Token::None]);
    }

    #[test]
    fn serialize_some() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Some(true).serialize(&serializer),
            [Token::Some, Token::Bool(true)]
        );
    }

    #[test]
    fn serialize_unit() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(().serialize(&serializer), [Token::Unit]);
    }

    #[test]
    fn serialize_unit_struct() {
        #[derive(Serialize)]
        struct Unit;

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Unit.serialize(&serializer),
            [Token::UnitStruct { name: "Unit" }]
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
            [Token::UnitVariant {
                name: "Unit",
                variant_index: 0,
                variant: "Variant"
            }]
        );
    }

    #[test]
    fn serialize_newtype_struct() {
        #[derive(Serialize)]
        struct Newtype(bool);

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            Newtype(false).serialize(&serializer),
            [Token::NewtypeStruct { name: "Newtype" }, Token::Bool(false)]
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
            [
                Token::NewtypeVariant {
                    name: "Newtype",
                    variant_index: 0,
                    variant: "Variant"
                },
                Token::Bool(false)
            ]
        );
    }

    #[test]
    fn serialize_seq() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            vec![1i8, 2i8, 3i8].serialize(&serializer),
            [
                Token::Seq { len: Some(3) },
                Token::I8(1),
                Token::I8(2),
                Token::I8(3),
                Token::SeqEnd
            ],
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
            [
                Token::Seq { len: Some(3) },
                Token::Unordered(&[
                    &[Token::Char('a')],
                    &[Token::Char('b')],
                    &[Token::Char('c')],
                ]),
                Token::SeqEnd,
            ]
        );
    }

    #[test]
    fn serialize_tuple() {
        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            (1i8, 2i16, 3i32).serialize(&serializer),
            [
                Token::Tuple { len: 3 },
                Token::I8(1),
                Token::I16(2),
                Token::I32(3),
                Token::TupleEnd
            ],
        );
    }

    #[test]
    fn serialize_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(i8, i16, i32);

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            TupleStruct(1i8, 2i16, 3i32).serialize(&serializer),
            [
                Token::TupleStruct {
                    name: "TupleStruct",
                    len: 3
                },
                Token::I8(1),
                Token::I16(2),
                Token::I32(3),
                Token::TupleStructEnd
            ],
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
            [
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
            ],
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
            [
                Token::Map { len: Some(3) },
                Token::Unordered(&[
                    &[Token::I8(1), Token::Char('a')],
                    &[Token::I8(2), Token::Char('b')],
                    &[Token::I8(3), Token::Char('c')],
                ]),
                Token::MapEnd,
            ]
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
            [
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
            ]
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
            [
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
            ]
        );
    }

    #[test]
    fn serialize_struct_as_seq() {
        #[derive(Serialize)]
        struct Struct {
            foo: bool,
            bar: u32,
        }

        let some_struct = Struct {
            foo: false,
            bar: 42,
        };
        let serializer = Serializer::builder()
            .serialize_struct_as(SerializeStructAs::Seq)
            .build();

        assert_ok_eq!(
            some_struct.serialize(&serializer),
            [
                Token::Seq { len: Some(2) },
                Token::Bool(false),
                Token::U32(42),
                Token::SeqEnd,
            ]
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
            [
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
            ]
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
            [
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
            ]
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
            [Token::Str("foo".to_owned())]
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
