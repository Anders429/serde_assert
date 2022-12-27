use crate::{Token, Tokens};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use core::fmt;
use core::fmt::Display;
use serde::ser;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::ser::SerializeStructVariant;
use serde::ser::SerializeTuple;
use serde::ser::SerializeTupleStruct;
use serde::ser::SerializeTupleVariant;
use serde::Serialize;

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
    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    is_human_readable: Option<bool>,
}

impl Builder {
    pub fn is_human_readable(&mut self, is_human_readable: bool) -> &mut Self {
        self.is_human_readable = Some(is_human_readable);
        self
    }

    pub fn build(&mut self) -> Serializer {
        Serializer {
            is_human_readable: self.is_human_readable.unwrap_or(true),
        }
    }
}

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
    use super::{Error, Serializer};
    use crate::{Token, Tokens};
    use alloc::{borrow::ToOwned, format, vec};
    use claims::assert_ok_eq;
    use serde::ser::Error as _;
    use serde::ser::Serialize;
    use serde::ser::Serializer as _;
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
