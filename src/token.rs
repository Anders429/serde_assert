use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashSet;

#[derive(Debug)]
pub enum Token {
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

    Unordered(&'static [&'static [Token]]),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Bool(a), Token::Bool(b)) => {
                a == b
            },
            (Token::I8(a), Token::I8(b)) => {
                a == b
            },
            (Token::I16(a), Token::I16(b)) => {
                a == b
            },
            (Token::I32(a), Token::I32(b)) => {
                a == b
            },
            (Token::I64(a), Token::I64(b)) => {
                a == b
            },
            #[cfg(has_i128)]
            (Token::I128(a), Token::I128(b)) => {
                a == b
            },
            (Token::U8(a), Token::U8(b)) => {
                a == b
            },
            (Token::U16(a), Token::U16(b)) => {
                a == b
            },
            (Token::U32(a), Token::U32(b)) => {
                a == b
            },
            (Token::U64(a), Token::U64(b)) => {
                a == b
            },
            #[cfg(has_i128)]
            (Token::U128(a), Token::U128(b)) => {
                a == b
            },
            (Token::F32(a), Token::F32(b)) => {
                a == b
            }
            (Token::F64(a), Token::F64(b)) => {
                a == b
            }
            (Token::Char(a), Token::Char(b)) => {
                a == b
            }
            (Token::Str(a), Token::Str(b)) => {
                a == b
            }
            (Token::Bytes(a), Token::Bytes(b)) => {
                a == b
            }
            (Token::None, Token::None) | (Token::Some, Token::Some) | (Token::Unit, Token::Unit) | (Token::SeqEnd, Token::SeqEnd) | (Token::TupleEnd, Token::TupleEnd) | (Token::TupleStructEnd, Token::TupleStructEnd) | (Token::TupleVariantEnd, Token::TupleVariantEnd) | (Token::MapEnd, Token::MapEnd) | (Token::StructEnd, Token::StructEnd) | (Token::StructVariantEnd, Token::StructVariantEnd) => true,
            (Token::UnitStruct {name: name_a}, Token::UnitStruct {name: name_b}) | (Token::NewtypeStruct {name: name_a}, Token::NewtypeStruct {name: name_b}) => {
                name_a == name_b
            }
            (Token::UnitVariant {name: name_a, variant_index: variant_index_a, variant: variant_a}, Token::UnitVariant {name: name_b, variant_index: variant_index_b, variant: variant_b}) | (Token::NewtypeVariant {name: name_a, variant_index: variant_index_a, variant: variant_a}, Token::NewtypeVariant {name: name_b, variant_index: variant_index_b, variant: variant_b}) => {
                name_a == name_b && variant_index_a == variant_index_b && variant_a == variant_b
            }
            (Token::Seq {len: len_a}, Token::Seq {len: len_b}) | (Token::Map {len: len_a}, Token::Map {len: len_b}) => {
                len_a == len_b
            }
            (Token::Tuple {len: len_a}, Token::Tuple {len: len_b}) => {
                len_a == len_b
            }
            (Token::TupleStruct {name: name_a, len: len_a}, Token::TupleStruct {name: name_b, len: len_b}) | (Token::Struct {name: name_a, len: len_a}, Token::Struct {name: name_b, len: len_b}) => {
                name_a == name_b && len_a == len_b
            }
            (Token::TupleVariant {name: name_a, variant_index: variant_index_a, variant: variant_a, len: len_a}, Token::TupleVariant {name: name_b, variant_index: variant_index_b, variant: variant_b, len: len_b}) | (Token::StructVariant {name: name_a, variant_index: variant_index_a, variant: variant_a, len: len_a}, Token::StructVariant {name: name_b, variant_index: variant_index_b, variant: variant_b, len: len_b}) => {
                name_a == name_b && variant_index_a == variant_index_b && variant_a == variant_b && len_a == len_b
            }
            (Token::Unordered(tokens_a), Token::Unordered(tokens_b)) => {
                if tokens_a.len() != tokens_b.len() {
                    return false;
                }

                let mut consumed = HashSet::new();
                'outer: for tokens in tokens_a.iter() {
                    for (i, other_tokens) in tokens_b.iter().enumerate().filter(|(i, _)| !consumed.contains(i)) {
                        if tokens == other_tokens {
                            consumed.insert(i);
                            continue 'outer;
                        }
                    }
                    return false;
                }
                true
            },
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Tokens(pub Vec<Token>);

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        let mut self_iter = self.0.iter();
        let mut other_iter = other.0.iter();

        loop {
            // Obtain next tokens, or return if no tokens are available.
            let self_token = match self_iter.next() {
                Some(token) => token,
                None => {
                    if let Some(_) = other_iter.next() {
                        return false;
                    } else {
                        return true;
                    }
                }
            };
            let other_token = match other_iter.next() {
                Some(token) => token,
                None => return false,
            };

            match (self_token, other_token) {
                (Token::Unordered(_), Token::Unordered(_)) => {
                    if self_token != other_token {
                        return false;
                    }
                },
                (Token::Unordered(tokens), _) => {
                    todo!()
                },
                (_, Token::Unordered(tokens)) => {
                    todo!()
                },
                _ => if self_token != other_token {
                    return false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Tokens};
    use alloc::{borrow::ToOwned, vec};

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
        assert_eq!(Token::UnitStruct {name: "a"}, Token::UnitStruct {name: "a"});
    }

    #[test]
    fn token_unit_struct_ne() {
        assert_ne!(Token::UnitStruct {name: "a"}, Token::UnitStruct {name: "b"});
    }

    #[test]
    fn token_unit_variant_eq() {
        assert_eq!(Token::UnitVariant {name: "a", variant_index: 1, variant: "foo"}, Token::UnitVariant {name: "a", variant_index: 1, variant: "foo"});
    }

    #[test]
    fn token_unit_variant_ne_name() {
        assert_ne!(Token::UnitVariant {name: "a", variant_index: 1, variant: "foo"}, Token::UnitVariant {name: "b", variant_index: 1, variant: "foo"});
    }

    #[test]
    fn token_unit_variant_ne_variant_index() {
        assert_ne!(Token::UnitVariant {name: "a", variant_index: 1, variant: "foo"}, Token::UnitVariant {name: "a", variant_index: 2, variant: "foo"});
    }

    #[test]
    fn token_unit_variant_ne_variant() {
        assert_ne!(Token::UnitVariant {name: "a", variant_index: 1, variant: "foo"}, Token::UnitVariant {name: "a", variant_index: 1, variant: "bar"});
    }

    #[test]
    fn token_newtype_struct_eq() {
        assert_eq!(Token::NewtypeStruct {name: "a"}, Token::NewtypeStruct {name: "a"});
    }

    #[test]
    fn token_newtype_struct_ne() {
        assert_ne!(Token::NewtypeStruct {name: "a"}, Token::NewtypeStruct {name: "b"});
    }

    #[test]
    fn token_newtype_variant_eq() {
        assert_eq!(Token::NewtypeVariant {name: "a", variant_index: 1, variant: "foo"}, Token::NewtypeVariant {name: "a", variant_index: 1, variant: "foo"});
    }

    #[test]
    fn token_newtype_variant_ne_name() {
        assert_ne!(Token::NewtypeVariant {name: "a", variant_index: 1, variant: "foo"}, Token::NewtypeVariant {name: "b", variant_index: 1, variant: "foo"});
    }

    #[test]
    fn token_newtype_variant_ne_variant_index() {
        assert_ne!(Token::NewtypeVariant {name: "a", variant_index: 1, variant: "foo"}, Token::NewtypeVariant {name: "a", variant_index: 2, variant: "foo"});
    }

    #[test]
    fn token_newtype_variant_ne_variant() {
        assert_ne!(Token::NewtypeVariant {name: "a", variant_index: 1, variant: "foo"}, Token::NewtypeVariant {name: "a", variant_index: 1, variant: "bar"});
    }

    #[test]
    fn token_seq_eq_some() {
        assert_eq!(Token::Seq {len: Some(42)}, Token::Seq {len: Some(42)});
    }

    #[test]
    fn token_seq_eq_none() {
        assert_eq!(Token::Seq {len: None}, Token::Seq {len: None});
    }

    #[test]
    fn token_seq_ne_some() {
        assert_ne!(Token::Seq {len: Some(42)}, Token::Seq {len: Some(43)});
    }

    #[test]
    fn token_seq_ne_some_none() {
        assert_ne!(Token::Seq {len: Some(42)}, Token::Seq {len: None});
    }

    #[test]
    fn token_seq_end_eq() {
        assert_eq!(Token::SeqEnd, Token::SeqEnd);
    }

    #[test]
    fn token_tuple_eq() {
        assert_eq!(Token::Tuple {len: 42}, Token::Tuple {len: 42});
    }

    #[test]
    fn token_tuple_ne() {
        assert_ne!(Token::Tuple {len: 42}, Token::Tuple {len: 43});
    }

    #[test]
    fn token_tuple_end_eq() {
        assert_eq!(Token::TupleEnd, Token::TupleEnd);
    }

    #[test]
    fn token_tuple_struct_eq() {
        assert_eq!(Token::TupleStruct {name: "a", len: 42}, Token::TupleStruct {name: "a", len: 42});
    }

    #[test]
    fn token_tuple_struct_ne_name() {
        assert_ne!(Token::TupleStruct {name: "a", len: 42}, Token::TupleStruct {name: "b", len: 42});
    }

    #[test]
    fn token_tuple_struct_ne_len() {
        assert_ne!(Token::TupleStruct {name: "a", len: 42}, Token::TupleStruct {name: "a", len: 43});
    }

    #[test]
    fn token_tuple_struct_end_eq() {
        assert_eq!(Token::TupleStructEnd, Token::TupleStructEnd);
    }

    #[test]
    fn token_tuple_variant_eq() {
        assert_eq!(Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 42});
    }

    #[test]
    fn token_tuple_variant_ne_name() {
        assert_ne!(Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::TupleVariant {name: "b", variant_index: 1, variant: "foo", len: 42});
    }

    #[test]
    fn token_tuple_variant_ne_variant_index() {
        assert_ne!(Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::TupleVariant {name: "a", variant_index: 2, variant: "foo", len: 42});
    }

    #[test]
    fn token_tuple_variant_ne_variant() {
        assert_ne!(Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::TupleVariant {name: "a", variant_index: 1, variant: "bar", len: 42});
    }

    #[test]
    fn token_tuple_variant_ne_len() {
        assert_ne!(Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::TupleVariant {name: "a", variant_index: 1, variant: "foo", len: 43});
    }

    #[test]
    fn token_tuple_variant_end_eq() {
        assert_eq!(Token::TupleVariantEnd, Token::TupleVariantEnd);
    }

    #[test]
    fn token_map_eq_some() {
        assert_eq!(Token::Map {len: Some(42)}, Token::Map {len: Some(42)});
    }

    #[test]
    fn token_map_eq_none() {
        assert_eq!(Token::Map {len: None}, Token::Map {len: None});
    }

    #[test]
    fn token_map_ne_some() {
        assert_ne!(Token::Map {len: Some(42)}, Token::Map {len: Some(43)});
    }

    #[test]
    fn token_map_ne_some_none() {
        assert_ne!(Token::Map {len: Some(42)}, Token::Map {len: None});
    }

    #[test]
    fn token_map_end_eq() {
        assert_eq!(Token::MapEnd, Token::MapEnd);
    }

    #[test]
    fn token_struct_eq() {
        assert_eq!(Token::Struct {name: "a", len: 42}, Token::Struct {name: "a", len: 42});
    }

    #[test]
    fn token_struct_ne_name() {
        assert_ne!(Token::Struct {name: "a", len: 42}, Token::Struct {name: "b", len: 42});
    }

    #[test]
    fn token_struct_ne_len() {
        assert_ne!(Token::Struct {name: "a", len: 42}, Token::Struct {name: "a", len: 43});
    }

    #[test]
    fn token_struct_end_eq() {
        assert_eq!(Token::StructEnd, Token::StructEnd);
    }

    #[test]
    fn token_struct_variant_ne_name() {
        assert_ne!(Token::StructVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::StructVariant {name: "b", variant_index: 1, variant: "foo", len: 42});
    }

    #[test]
    fn token_struct_variant_ne_variant_index() {
        assert_ne!(Token::StructVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::StructVariant {name: "a", variant_index: 2, variant: "foo", len: 42});
    }

    #[test]
    fn token_struct_variant_ne_variant() {
        assert_ne!(Token::StructVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::StructVariant {name: "a", variant_index: 1, variant: "bar", len: 42});
    }

    #[test]
    fn token_struct_variant_ne_len() {
        assert_ne!(Token::StructVariant {name: "a", variant_index: 1, variant: "foo", len: 42}, Token::StructVariant {name: "a", variant_index: 1, variant: "foo", len: 43});
    }

    #[test]
    fn token_struct_variant_end_eq() {
        assert_eq!(Token::StructVariantEnd, Token::StructVariantEnd);
    }

    #[test]
    fn token_variant_ne() {
        assert_ne!(Token::Bool(true), Token::U16(42));
    }

    #[test]
    fn tokens_bool_eq() {
        assert_eq!(Tokens(vec![Token::Bool(true)]), Tokens(vec![Token::Bool(true)]));
    }

    #[test]
    fn tokens_bool_ne() {
        assert_ne!(Tokens(vec![Token::Bool(true)]), Tokens(vec![Token::Bool(false)]));
    }

    #[test]
    fn tokens_variant_ne() {
        assert_ne!(Tokens(vec![Token::Bool(true)]), Tokens(vec![Token::U16(42)]));
    }
}
