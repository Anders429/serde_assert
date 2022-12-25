use crate::Token;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Display;
use serde::ser;
use serde::ser::SerializeSeq;
use serde::ser::SerializeTuple;
use serde::ser::SerializeTupleStruct;
use serde::ser::SerializeTupleVariant;
use serde::ser::SerializeMap;
use serde::ser::SerializeStruct;
use serde::ser::SerializeStructVariant;
use serde::Serialize;

#[derive(Debug)]
pub struct Serializer {
    is_human_readable: bool,
}

impl<'a> ser::Serializer for &'a Serializer {
    type Ok = Vec<Token>;
    type Error = Error;

    type SerializeSeq = CompoundSerializer<'a>;
    type SerializeTuple = CompoundSerializer<'a>;
    type SerializeTupleStruct = CompoundSerializer<'a>;
    type SerializeTupleVariant = CompoundSerializer<'a>;
    type SerializeMap = CompoundSerializer<'a>;
    type SerializeStruct = CompoundSerializer<'a>;
    type SerializeStructVariant = CompoundSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::Bool(v)])
    }

    fn serialize_i8(self, v: i8) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::I8(v)])
    }

    fn serialize_i16(self, v: i16) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::I16(v)])
    }

    fn serialize_i32(self, v: i32) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::I32(v)])
    }

    fn serialize_i64(self, v: i64) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::I64(v)])
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::I128(v)])
    }

    fn serialize_u8(self, v: u8) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::U8(v)])
    }

    fn serialize_u16(self, v: u16) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::U16(v)])
    }

    fn serialize_u32(self, v: u32) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::U32(v)])
    }

    fn serialize_u64(self, v: u64) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::U64(v)])
    }

    #[cfg(has_i128)]
    fn serialize_u128(self, v: u128) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::U128(v)])
    }

    fn serialize_f32(self, v: f32) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::F32(v)])
    }

    fn serialize_f64(self, v: f64) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::F64(v)])
    }

    fn serialize_char(self, v: char) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::Char(v)])
    }

    fn serialize_str(self, v: &str) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::Str(v.to_owned())])
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::Bytes(v.to_owned())])
    }

    fn serialize_none(self) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::None])
    }

    fn serialize_some<T>(self, value: &T) -> Result<Vec<Token>, Error> where T: Serialize + ?Sized {
        let mut tokens = vec![Token::Some];
        tokens.extend(value.serialize(self)?);
        Ok(tokens)
    }

    fn serialize_unit(self) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::Unit])
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::UnitStruct {
            name,
        }])
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Vec<Token>, Error> {
        Ok(vec![Token::UnitVariant {
            name,
            variant_index,
            variant,
        }])
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Vec<Token>, Error> where T: Serialize + ?Sized {
        let mut tokens = vec![Token::NewtypeStruct {
            name,
        }];
        tokens.extend(value.serialize(self)?);
        Ok(tokens)
    }

    fn serialize_newtype_variant<T>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Vec<Token>, Error> where T: Serialize + ?Sized {
        let mut tokens = vec![Token::NewtypeVariant {
            name,
            variant_index,
            variant,
        }];
        tokens.extend(value.serialize(self)?);
        Ok(tokens)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::Seq {len,}],

            serializer: self,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::Tuple {len,}],

            serializer: self,
        })
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::TupleStruct {name, len,}],

            serializer: self,
        })
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::TupleVariant {name, variant_index, variant, len,}],

            serializer: self,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::Map {len,}],

            serializer: self,
        })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::Struct {name, len}],

            serializer: self,
        })
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<CompoundSerializer<'a>, Error> {
        Ok(CompoundSerializer {
            tokens: vec![Token::StructVariant {name, variant_index, variant, len}],

            serializer: self,
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Vec<Token>, Error> where T: Display + ?Sized {
        Ok(vec![Token::Str(value.to_string())])
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
    tokens: Vec<Token>,

    serializer: &'a Serializer,
}

impl SerializeSeq for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::SeqEnd);
        Ok(self.tokens)
    }
}

impl SerializeTuple for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::TupleEnd);
        Ok(self.tokens)
    }
}

impl SerializeTupleStruct for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::TupleStructEnd);
        Ok(self.tokens)
    }
}

impl SerializeTupleVariant for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::TupleVariantEnd);
        Ok(self.tokens)
    }
}

impl SerializeMap for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_key<T>(&mut self, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::MapEnd);
        Ok(self.tokens)
    }
}

impl SerializeStruct for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.push(Token::Field(key));
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.tokens.push(Token::SkippedField(key));
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::StructEnd);
        Ok(self.tokens)
    }
}

impl SerializeStructVariant for CompoundSerializer<'_> {
    type Ok = Vec<Token>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error> where T: Serialize + ?Sized {
        self.tokens.push(Token::Field(key));
        self.tokens.extend(value.serialize(self.serializer)?);
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.tokens.push(Token::SkippedField(key));
        Ok(())
    }

    fn end(mut self) -> Result<Vec<Token>, Error> {
        self.tokens.push(Token::StructVariantEnd);
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
    fn custom<T>(msg: T) -> Self where T: Display {
        Self(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Serializer, Error};
    use alloc::format;
    use serde::ser::Serializer as _;
    use serde::ser::Error as _;

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
