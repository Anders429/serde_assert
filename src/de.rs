use crate::{Token, Tokens};
use alloc::{
    string::{String, ToString},
    vec,
};
use core::{fmt, fmt::Display};
use serde::de::Error as _;
use serde::{
    de,
    de::{DeserializeSeed, Expected, Unexpected},
};

#[derive(Debug)]
pub struct Deserializer {
    tokens: vec::IntoIter<Token>,

    revisited_token: Option<Token>,

    is_human_readable: bool,
    self_describing: bool,
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if !self.self_describing {
            return Err(Error::NotSelfDescribing);
        }
        let token = self.next_token()?;
        match token {
            Token::Bool(v) => visitor.visit_bool(v),
            Token::I8(v) => visitor.visit_i8(v),
            Token::I16(v) => visitor.visit_i16(v),
            Token::I32(v) => visitor.visit_i32(v),
            Token::I64(v) => visitor.visit_i64(v),
            #[cfg(has_i128)]
            Token::I128(v) => visitor.visit_i128(v),
            Token::U8(v) => visitor.visit_u8(v),
            Token::U16(v) => visitor.visit_u16(v),
            Token::U32(v) => visitor.visit_u32(v),
            Token::U64(v) => visitor.visit_u64(v),
            #[cfg(has_i128)]
            Token::U128(v) => visitor.visit_u128(v),
            Token::F32(v) => visitor.visit_f32(v),
            Token::F64(v) => visitor.visit_f64(v),
            Token::Char(v) => visitor.visit_char(v),
            Token::Str(v) => visitor.visit_string(v),
            Token::Bytes(v) => visitor.visit_byte_buf(v),
            Token::None => visitor.visit_none(),
            Token::Some => visitor.visit_some(self),
            Token::Unit | Token::UnitStruct { .. } => visitor.visit_unit(),
            Token::UnitVariant { .. }
            | Token::NewtypeVariant { .. }
            | Token::TupleVariant { .. }
            | Token::StructVariant { .. } => {
                // `EnumDeserializer` takes care of the enum deserialization, which will consume this token later.
                self.revisit_token(token);
                visitor.visit_enum(EnumAccess { deserializer: self })
            }
            Token::NewtypeStruct { .. } => visitor.visit_newtype_struct(self),
            Token::Seq { len } => {
                let mut access = SeqAccess {
                    deserializer: self,

                    len,

                    end_token: Token::SeqEnd,
                    ended: false,
                };
                let result = visitor.visit_seq(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
            Token::Tuple { len } => {
                let mut access = SeqAccess {
                    deserializer: self,

                    len: Some(len),

                    end_token: Token::TupleEnd,
                    ended: false,
                };
                let result = visitor.visit_seq(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
            Token::TupleStruct { name: _, len } => {
                let mut access = SeqAccess {
                    deserializer: self,

                    len: Some(len),

                    end_token: Token::TupleStructEnd,
                    ended: false,
                };
                let result = visitor.visit_seq(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
            Token::Map { len } => {
                let mut access = MapAccess {
                    deserializer: self,

                    len,

                    end_token: Token::MapEnd,
                    ended: false,
                };
                let result = visitor.visit_map(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
            Token::Field(v) => visitor.visit_str(v),
            Token::Struct { name: _, len } => {
                let mut access = MapAccess {
                    deserializer: self,

                    len: Some(len),

                    end_token: Token::StructEnd,
                    ended: false,
                };
                let result = visitor.visit_map(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
            _ => Err(Self::Error::invalid_type((&token).into(), &visitor)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Bool(v) = token {
            visitor.visit_bool(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I8(v) = token {
            visitor.visit_i8(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I16(v) = token {
            visitor.visit_i16(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I32(v) = token {
            visitor.visit_i32(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I64(v) = token {
            visitor.visit_i64(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I128(v) = token {
            visitor.visit_i128(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U8(v) = token {
            visitor.visit_u8(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U16(v) = token {
            visitor.visit_u16(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U32(v) = token {
            visitor.visit_u32(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U64(v) = token {
            visitor.visit_u64(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U128(v) = token {
            visitor.visit_u128(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::F32(v) = token {
            visitor.visit_f32(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::F64(v) = token {
            visitor.visit_f64(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Char(v) = token {
            visitor.visit_char(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Str(v) = token {
            visitor.visit_str(&v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Str(v) = token {
            visitor.visit_string(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Bytes(v) = token {
            visitor.visit_bytes(&v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Bytes(v) = token {
            visitor.visit_byte_buf(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.next_token()? {
            Token::Some => visitor.visit_some(self),
            Token::None => visitor.visit_none(),
            token => Err(Self::Error::invalid_type((&token).into(), &visitor)),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Unit = token {
            visitor.visit_unit()
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::UnitStruct { name: struct_name } = token {
            if name == struct_name {
                visitor.visit_unit()
            } else {
                Err(Self::Error::invalid_value((&token).into(), &visitor))
            }
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::NewtypeStruct { name: struct_name } = token {
            if name == struct_name {
                visitor.visit_newtype_struct(self)
            } else {
                Err(Self::Error::invalid_value((&token).into(), &visitor))
            }
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Seq { len } = token {
            let mut access = SeqAccess {
                deserializer: self,

                len,

                end_token: Token::SeqEnd,
                ended: false,
            };
            let result = visitor.visit_seq(&mut access)?;
            access.assert_ended()?;
            Ok(result)
        } else {
            Err(Self::Error::invalid_value((&token).into(), &visitor))
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Tuple { len: token_len } = token {
            if len != token_len {
                Err(Self::Error::invalid_length(token_len, &visitor))
            } else {
                let mut access = SeqAccess {
                    deserializer: self,

                    len: Some(len),

                    end_token: Token::TupleEnd,
                    ended: false,
                };
                let result = visitor.visit_seq(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
        } else {
            Err(Self::Error::invalid_value((&token).into(), &visitor))
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::TupleStruct {
            name: token_name,
            len: token_len,
        } = token
        {
            if name != token_name {
                Err(Self::Error::invalid_value((&token).into(), &visitor))
            } else if len != token_len {
                Err(Self::Error::invalid_length(token_len, &visitor))
            } else {
                let mut access = SeqAccess {
                    deserializer: self,

                    len: Some(len),

                    end_token: Token::TupleStructEnd,
                    ended: false,
                };
                let result = visitor.visit_seq(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
        } else {
            Err(Self::Error::invalid_value((&token).into(), &visitor))
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Map { len } = token {
            let mut access = SeqAccess {
                deserializer: self,

                len,

                end_token: Token::TupleEnd,
                ended: false,
            };
            let result = visitor.visit_seq(&mut access)?;
            access.assert_ended()?;
            Ok(result)
        } else {
            Err(Self::Error::invalid_value((&token).into(), &visitor))
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Struct {
            name: token_name,
            len,
        } = token
        {
            if name != token_name {
                Err(Self::Error::invalid_value((&token).into(), &visitor))
            } else {
                let mut access = SeqAccess {
                    deserializer: self,

                    len: Some(len),

                    end_token: Token::TupleEnd,
                    ended: false,
                };
                let result = visitor.visit_seq(&mut access)?;
                access.assert_ended()?;
                Ok(result)
            }
        } else {
            Err(Self::Error::invalid_value((&token).into(), &visitor))
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        match token {
            Token::UnitVariant {
                name: token_name, ..
            }
            | Token::NewtypeVariant {
                name: token_name, ..
            }
            | Token::TupleVariant {
                name: token_name, ..
            }
            | Token::StructVariant {
                name: token_name, ..
            } => {
                if name != token_name {
                    Err(Self::Error::invalid_value((&token).into(), &visitor))
                } else {
                    // `EnumDeserializer` takes care of the enum deserialization, which will consume this token later.
                    self.revisit_token(token);
                    visitor.visit_enum(EnumAccess { deserializer: self })
                }
            }
            _ => Err(Self::Error::invalid_type((&token).into(), &visitor)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        match token {
            Token::Str(v) => visitor.visit_str(&v),
            Token::Field(v) => visitor.visit_str(v),
            _ => Err(Self::Error::invalid_type((&token).into(), &visitor)),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

impl Deserializer {
    pub fn builder() -> Builder {
        Builder::default()
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        loop {
            let token = self
                .revisited_token
                .take()
                .into_iter()
                .chain(&mut self.tokens)
                .next()
                .ok_or(Error::EndOfTokens)?;
            if !matches!(token, Token::SkippedField(_)) {
                return Ok(token);
            }
        }
    }

    fn revisit_token(&mut self, token: Token) {
        self.revisited_token = Some(token);
    }
}

struct SeqAccess<'a> {
    deserializer: &'a mut Deserializer,

    len: Option<usize>,

    end_token: Token,
    ended: bool,
}

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.ended {
            return Ok(None);
        }
        let token = self.deserializer.next_token()?;
        if token == self.end_token {
            self.ended = true;
            return Ok(None);
        }
        self.deserializer.revisit_token(token);
        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.len
    }
}

impl SeqAccess<'_> {
    fn assert_ended(&mut self) -> Result<(), Error> {
        if !self.ended {
            if self.deserializer.next_token()? != self.end_token {
                return Err(Error::ExpectedToken(self.end_token.clone()));
            }
        }
        self.ended = true;
        Ok(())
    }
}

struct MapAccess<'a> {
    deserializer: &'a mut Deserializer,

    len: Option<usize>,

    end_token: Token,
    ended: bool,
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.ended {
            return Ok(None);
        }
        let token = self.deserializer.next_token()?;
        if token == self.end_token {
            self.ended = true;
            return Ok(None);
        }
        self.deserializer.revisit_token(token);
        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }

    fn size_hint(&self) -> Option<usize> {
        self.len
    }
}

impl MapAccess<'_> {
    fn assert_ended(&mut self) -> Result<(), Error> {
        if !self.ended {
            if self.deserializer.next_token()? != self.end_token {
                return Err(Error::ExpectedToken(self.end_token.clone()));
            }
        }
        self.ended = true;
        Ok(())
    }
}

struct EnumAccess<'a> {
    deserializer: &'a mut Deserializer,
}

impl<'a, 'de> de::EnumAccess<'de> for EnumAccess<'a> {
    type Error = Error;
    type Variant = VariantAccess<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(EnumDeserializer {
            deserializer: self.deserializer,
        })?;
        Ok((
            value,
            VariantAccess {
                deserializer: self.deserializer,
            },
        ))
    }
}

struct VariantAccess<'a> {
    deserializer: &'a mut Deserializer,
}

impl<'a, 'de> de::VariantAccess<'de> for VariantAccess<'a> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess {
            deserializer: self.deserializer,

            len: Some(len),

            end_token: Token::TupleVariantEnd,
            ended: false,
        })
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(MapAccess {
            deserializer: self.deserializer,

            len: None,

            end_token: Token::StructVariantEnd,
            ended: false,
        })
    }
}

/// Wrapper around `Deserializer` to deserialize enum tokens directly, rather than using
/// `EnumAccess`.
///
/// This is required to ensure the token can be properly deserialized into a variant.
struct EnumDeserializer<'a> {
    deserializer: &'a mut Deserializer,
}

impl<'a, 'de> de::Deserializer<'de> for EnumDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.deserializer.next_token()? {
            Token::UnitVariant { variant, .. }
            | Token::TupleVariant { variant, .. }
            | Token::NewtypeVariant { variant, .. }
            | Token::StructVariant { variant, .. } => visitor.visit_str(variant),
            _ => unreachable!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.deserializer.next_token()? {
            Token::UnitVariant { variant_index, .. }
            | Token::TupleVariant { variant_index, .. }
            | Token::NewtypeVariant { variant_index, .. }
            | Token::StructVariant { variant_index, .. } => visitor.visit_u32(variant_index),
            _ => unreachable!(),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        self.deserializer.is_human_readable()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    tokens: Option<Tokens>,

    is_human_readable: Option<bool>,
    self_describing: Option<bool>,
}

impl Builder {
    pub fn tokens(&mut self, tokens: Tokens) -> &mut Self {
        self.tokens = Some(tokens);
        self
    }

    pub fn is_human_readable(&mut self, is_human_readable: bool) -> &mut Self {
        self.is_human_readable = Some(is_human_readable);
        self
    }

    pub fn self_describing(&mut self, self_describing: bool) -> &mut Self {
        self.self_describing = Some(self_describing);
        self
    }

    pub fn build(&mut self) -> Deserializer {
        Deserializer {
            tokens: self
                .tokens
                .clone()
                .expect("no tokens provided to `Deserializer` `Builder`")
                .0
                .into_iter(),

            revisited_token: None,

            is_human_readable: self.is_human_readable.unwrap_or(true),
            self_describing: self.self_describing.unwrap_or(true),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EndOfTokens,

    ExpectedToken(Token),

    NotSelfDescribing,

    Custom(String),
    InvalidType(String, String),
    InvalidValue(String, String),
    InvalidLength(usize, String),
    UnknownVariant(String, &'static [&'static str]),
    UnknownField(String, &'static [&'static str]),
    MissingField(&'static str),
    DuplicateField(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndOfTokens => f.write_str("end of tokens"),
            Self::ExpectedToken(token) => write!(f, "expected token {}", token),
            Self::NotSelfDescribing => f.write_str("attempted to deserialize as self-describing when deserializer is not set as self-describing"),
            Self::Custom(s) => f.write_str(s),
            Self::InvalidType(unexpected, expected) => write!(f, "invalid type: expected {}, found {}", expected, unexpected),
            Self::InvalidValue(unexpected, expected) => write!(f, "invalid value: expected {}, found {}", expected, unexpected),
            Self::InvalidLength(length, expected) => write!(f, "invalid length {}, expected {}", length, expected),
            Self::UnknownVariant(variant, expected) => write!(f, "unknown variant {}, expected one of {:?}", variant, expected),
            Self::UnknownField(field, expected) => write!(f, "unknown field {}, expected one of {:?}", field, expected),
            Self::MissingField(field) => write!(f, "missing field {}", field),
            Self::DuplicateField(field) => write!(f, "duplicate field {}", field),
        }
    }
}

impl de::StdError for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }

    fn invalid_type(unexpected: Unexpected, expected: &dyn Expected) -> Self {
        Self::InvalidType(unexpected.to_string(), expected.to_string())
    }

    fn invalid_value(unexpected: Unexpected, expected: &dyn Expected) -> Self {
        Self::InvalidValue(unexpected.to_string(), expected.to_string())
    }

    fn invalid_length(len: usize, expected: &dyn Expected) -> Self {
        Self::InvalidLength(len, expected.to_string())
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self::UnknownVariant(variant.to_string(), expected)
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::UnknownField(field.to_string(), expected)
    }

    fn missing_field(field: &'static str) -> Self {
        Self::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self::DuplicateField(field)
    }
}

#[cfg(test)]
mod tests {
    use super::{Deserializer, Error};
    use crate::{Token, Tokens};
    use alloc::{borrow::ToOwned, fmt, format, string::String, vec, vec::Vec};
    use claims::{assert_err_eq, assert_ok_eq};
    use serde::de;
    use serde::de::{Deserialize, IgnoredAny, Unexpected, Visitor};
    use serde::de::{Error as _, VariantAccess};

    #[derive(Debug, PartialEq)]
    enum Any {
        Bool(bool),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
        #[cfg(has_i128)]
        I128(i128),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        #[cfg(has_i128)]
        U128(u128),
        F32(f32),
        F64(f64),
        Char(char),
        Str(String),
        Bytes(Vec<u8>),
        Option(Option<u32>),
        Unit,
        UnitVariant,
        NewtypeStruct(u32),
        NewtypeVariant(u32),
        Seq(u32, u32, u32),
        TupleVariant(u32, u32, u32),
        Map {
            foo: u32,
            bar: bool,
        },
    }

    impl<'de> Deserialize<'de> for Any {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct AnyVisitor;

            impl<'de> Visitor<'de> for AnyVisitor {
                type Value = Any;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct Any")
                }

                fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::Bool(v))
                }

                fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::I8(v))
                }

                fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::I16(v))
                }

                fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::I32(v))
                }

                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::I64(v))
                }

                #[cfg(has_i128)]
                fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::I128(v))
                }

                fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::U8(v))
                }

                fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::U16(v))
                }

                fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::U32(v))
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::U64(v))
                }

                #[cfg(has_i128)]
                fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::U128(v))
                }

                fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::F32(v))
                }

                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::F64(v))
                }

                fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::Char(v))
                }

                fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::Str(v))
                }

                fn visit_byte_buf<E>(self, v: vec::Vec<u8>) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::Bytes(v))
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    if let Any::U32(v) = deserializer.deserialize_any(self)? {
                        Ok(Any::Option(Some(v)))
                    } else {
                        unreachable!()
                    }
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::Option(None))
                }

                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Any::Unit)
                }

                fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::EnumAccess<'de>,
                {
                    enum Variant {
                        Unit,
                        Newtype,
                        Tuple,
                    }

                    impl<'de> Deserialize<'de> for Variant {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct VariantVisitor;

                            impl<'de> Visitor<'de> for VariantVisitor {
                                type Value = Variant;

                                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                    formatter.write_str("enum Variant")
                                }

                                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                where
                                    E: de::Error,
                                {
                                    match v {
                                        "unit" => Ok(Variant::Unit),
                                        "newtype" => Ok(Variant::Newtype),
                                        "tuple" => Ok(Variant::Tuple),
                                        _ => Err(E::invalid_value(Unexpected::Str(v), &self)),
                                    }
                                }
                            }

                            deserializer.deserialize_any(VariantVisitor)
                        }
                    }

                    let (variant, access) = data.variant()?;

                    match variant {
                        Variant::Unit => {
                            access.unit_variant()?;
                            Ok(Any::UnitVariant)
                        }
                        Variant::Newtype => {
                            if let Any::U32(v) = access.newtype_variant()? {
                                Ok(Any::NewtypeVariant(v))
                            } else {
                                unreachable!()
                            }
                        }
                        Variant::Tuple => {
                            if let Any::Seq(a, b, c) = access.tuple_variant(3, self)? {
                                Ok(Any::TupleVariant(a, b, c))
                            } else {
                                unreachable!()
                            }
                        }
                    }
                }

                fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    if let Any::U32(v) = deserializer.deserialize_any(self)? {
                        Ok(Any::NewtypeStruct(v))
                    } else {
                        unreachable!()
                    }
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
                {
                    Ok(Any::Seq(
                        seq.next_element()?
                            .ok_or(A::Error::invalid_length(0, &self))?,
                        seq.next_element()?
                            .ok_or(A::Error::invalid_length(1, &self))?,
                        seq.next_element()?
                            .ok_or(A::Error::invalid_length(2, &self))?,
                    ))
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    enum Field {
                        Foo,
                        Bar,
                    }

                    impl<'de> Deserialize<'de> for Field {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct FieldVisitor;

                            impl<'de> Visitor<'de> for FieldVisitor {
                                type Value = Field;

                                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                    formatter.write_str("`foo` or `bar`")
                                }

                                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                                where
                                    E: de::Error,
                                {
                                    match value {
                                        "foo" => Ok(Field::Foo),
                                        "bar" => Ok(Field::Bar),
                                        _ => Err(E::unknown_field(value, &["foo", "bar"])),
                                    }
                                }
                            }

                            deserializer.deserialize_identifier(FieldVisitor)
                        }
                    }

                    let mut foo = None;
                    let mut bar = None;

                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Foo => {
                                if foo.is_some() {
                                    return Err(A::Error::duplicate_field("foo"));
                                }
                                foo = Some(map.next_value()?);
                            }
                            Field::Bar => {
                                if bar.is_some() {
                                    return Err(A::Error::duplicate_field("bar"));
                                }
                                bar = Some(map.next_value()?);
                            }
                        }
                    }

                    if foo.is_none() {
                        return Err(A::Error::missing_field("foo"));
                    }
                    if bar.is_none() {
                        return Err(A::Error::missing_field("bar"));
                    }

                    Ok(Any::Map {
                        foo: foo.unwrap(),
                        bar: bar.unwrap(),
                    })
                }
            }

            deserializer.deserialize_any(AnyVisitor)
        }
    }

    #[test]
    fn deserialize_any_bool() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Bool(true)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Bool(true));
    }

    #[test]
    fn deserialize_any_i8() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::I8(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::I8(42));
    }

    #[test]
    fn deserialize_any_i16() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::I16(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::I16(42));
    }

    #[test]
    fn deserialize_any_i32() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::I32(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::I32(42));
    }

    #[test]
    fn deserialize_any_i64() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::I64(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::I64(42));
    }

    #[cfg(has_i128)]
    #[test]
    fn deserialize_any_i128() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::I128(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::I128(42));
    }

    #[test]
    fn deserialize_any_u8() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::U8(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::U8(42));
    }

    #[test]
    fn deserialize_any_u16() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::U16(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::U16(42));
    }

    #[test]
    fn deserialize_any_u32() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::U32(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::U32(42));
    }

    #[test]
    fn deserialize_any_u64() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::U64(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::U64(42));
    }

    #[cfg(has_i128)]
    #[test]
    fn deserialize_any_u128() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::U128(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::U128(42));
    }

    #[test]
    fn deserialize_any_f32() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::F32(42.)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::F32(42.));
    }

    #[test]
    fn deserialize_any_f64() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::F64(42.)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::F64(42.));
    }

    #[test]
    fn deserialize_any_char() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Char('a')]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Char('a'));
    }

    #[test]
    fn deserialize_any_str() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Str("foo".to_owned())]))
            .build();

        assert_ok_eq!(
            Any::deserialize(&mut deserializer),
            Any::Str("foo".to_owned())
        );
    }

    #[test]
    fn deserialize_any_bytes() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Bytes(b"foo".to_vec())]))
            .build();

        assert_ok_eq!(
            Any::deserialize(&mut deserializer),
            Any::Bytes(b"foo".to_vec())
        );
    }

    #[test]
    fn deserialize_any_some() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Some, Token::U32(42)]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Option(Some(42)),);
    }

    #[test]
    fn deserialize_any_none() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::None]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Option(None),);
    }

    #[test]
    fn deserialize_any_unit() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Unit]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Unit,);
    }

    #[test]
    fn deserialize_any_unit_struct() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::UnitStruct { name: "foo" }]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Unit,);
    }

    #[test]
    fn deserialize_any_unit_variant() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::UnitVariant {
                name: "foo",
                variant_index: 0,
                variant: "unit",
            }]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::UnitVariant,);
    }

    #[test]
    fn deserialize_any_newtype_struct() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::NewtypeStruct { name: "foo" },
                Token::U32(42),
            ]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::NewtypeStruct(42),);
    }

    #[test]
    fn deserialize_any_newtype_variant() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::NewtypeVariant {
                    name: "foo",
                    variant_index: 0,
                    variant: "newtype",
                },
                Token::U32(42),
            ]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::NewtypeVariant(42),);
    }

    #[test]
    fn deserialize_any_seq() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::Seq { len: None },
                Token::U32(1),
                Token::U32(2),
                Token::U32(3),
                Token::SeqEnd,
            ]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Seq(1, 2, 3),);
    }

    #[test]
    fn deserialize_any_tuple() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::Tuple { len: 3 },
                Token::U32(1),
                Token::U32(2),
                Token::U32(3),
                Token::TupleEnd,
            ]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Seq(1, 2, 3),);
    }

    #[test]
    fn deserialize_any_tuple_struct() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::TupleStruct {
                    name: "foo",
                    len: 3,
                },
                Token::U32(1),
                Token::U32(2),
                Token::U32(3),
                Token::TupleStructEnd,
            ]))
            .build();

        assert_ok_eq!(Any::deserialize(&mut deserializer), Any::Seq(1, 2, 3),);
    }

    #[test]
    fn deserialize_any_tuple_variant() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::TupleVariant {
                    name: "foo",
                    variant_index: 0,
                    variant: "tuple",
                    len: 3,
                },
                Token::U32(1),
                Token::U32(2),
                Token::U32(3),
                Token::TupleVariantEnd,
            ]))
            .build();

        assert_ok_eq!(
            Any::deserialize(&mut deserializer),
            Any::TupleVariant(1, 2, 3),
        );
    }

    #[test]
    fn deserialize_any_map() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::Map { len: Some(3) },
                Token::Str("foo".to_owned()),
                Token::U32(42),
                Token::Str("bar".to_owned()),
                Token::Bool(false),
                Token::MapEnd,
            ]))
            .build();

        assert_ok_eq!(
            Any::deserialize(&mut deserializer),
            Any::Map {
                foo: 42,
                bar: false
            },
        );
    }

    #[test]
    fn deserialize_any_struct() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![
                Token::Struct {
                    name: "foo",
                    len: 3,
                },
                Token::Field("foo"),
                Token::U32(42),
                Token::Field("bar"),
                Token::Bool(false),
                Token::StructEnd,
            ]))
            .build();

        assert_ok_eq!(
            Any::deserialize(&mut deserializer),
            Any::Map {
                foo: 42,
                bar: false
            },
        );
    }

    #[test]
    fn deserialize_any_not_self_describing() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Bool(true)]))
            .self_describing(false)
            .build();

        assert_err_eq!(
            Any::deserialize(&mut deserializer),
            Error::NotSelfDescribing
        );
    }

    #[test]
    fn deserialize_ignored_any_not_self_describing() {
        let mut deserializer = Deserializer::builder()
            .tokens(Tokens(vec![Token::Bool(true)]))
            .self_describing(false)
            .build();

        assert_err_eq!(
            IgnoredAny::deserialize(&mut deserializer),
            Error::NotSelfDescribing
        );
    }

    #[test]
    fn display_error_end_of_tokens() {
        assert_eq!(format!("{}", Error::EndOfTokens), "end of tokens");
    }

    #[test]
    fn display_error_expected_seq_end() {
        assert_eq!(
            format!("{}", Error::ExpectedToken(Token::SeqEnd)),
            "expected token SeqEnd"
        );
    }

    #[test]
    fn display_error_expected_tuple_end() {
        assert_eq!(
            format!("{}", Error::ExpectedToken(Token::TupleEnd)),
            "expected token TupleEnd"
        );
    }

    #[test]
    fn display_error_not_self_describing() {
        assert_eq!(format!("{}", Error::NotSelfDescribing), "attempted to deserialize as self-describing when deserializer is not set as self-describing");
    }

    #[test]
    fn display_error_custom() {
        assert_eq!(format!("{}", Error::Custom("foo".to_owned())), "foo");
    }

    #[test]
    fn display_error_invalid_type() {
        assert_eq!(
            format!(
                "{}",
                Error::invalid_type((&Token::Bool(true)).into(), &"foo")
            ),
            "invalid type: expected foo, found boolean `true`"
        );
    }

    #[test]
    fn display_error_invalid_value() {
        assert_eq!(
            format!(
                "{}",
                Error::invalid_value((&Token::Bool(true)).into(), &"foo")
            ),
            "invalid value: expected foo, found boolean `true`"
        );
    }

    #[test]
    fn display_error_invalid_length() {
        assert_eq!(
            format!("{}", Error::invalid_length(42, &"foo")),
            "invalid length 42, expected foo"
        );
    }

    #[test]
    fn display_error_unknown_variant() {
        assert_eq!(
            format!("{}", Error::unknown_variant("foo", &["bar", "baz"])),
            "unknown variant foo, expected one of [\"bar\", \"baz\"]"
        );
    }

    #[test]
    fn display_error_unknown_field() {
        assert_eq!(
            format!("{}", Error::unknown_field("foo", &["bar", "baz"])),
            "unknown field foo, expected one of [\"bar\", \"baz\"]"
        );
    }

    #[test]
    fn display_error_missing_field() {
        assert_eq!(
            format!("{}", Error::missing_field("foo")),
            "missing field foo"
        );
    }

    #[test]
    fn display_error_duplicate_field() {
        assert_eq!(
            format!("{}", Error::duplicate_field("foo")),
            "duplicate field foo"
        );
    }
}
