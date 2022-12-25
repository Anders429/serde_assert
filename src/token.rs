use alloc::string::String;
use alloc::vec::Vec;

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
                (Token::Unordered(tokens), _) => {
                    todo!()
                },
                (_, Token::Unordered(tokens)) => {
                    todo!()
                },
                (Token::Bool(a), Token::Bool(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::I8(a), Token::I8(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::I16(a), Token::I16(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::I32(a), Token::I32(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::I64(a), Token::I64(b)) => {
                    if a != b {
                        return false;
                    }
                },
                #[cfg(has_i128)]
                (Token::I128(a), Token::I128(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::U8(a), Token::U8(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::U16(a), Token::U16(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::U32(a), Token::U32(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::U64(a), Token::U64(b)) => {
                    if a != b {
                        return false;
                    }
                },
                #[cfg(has_i128)]
                (Token::U128(a), Token::U128(b)) => {
                    if a != b {
                        return false;
                    }
                },
                (Token::Char(a), Token::Char(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (Token::Str(a), Token::Str(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (Token::Bytes(a), Token::Bytes(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (Token::None, Token::None) | (Token::Some, Token::Some) | (Token::Unit, Token::Unit) | (Token::SeqEnd, Token::SeqEnd) | (Token::TupleEnd, Token::TupleEnd) | (Token::TupleStructEnd, Token::TupleStructEnd) | (Token::TupleVariantEnd, Token::TupleVariantEnd) | (Token::MapEnd, Token::MapEnd) | (Token::StructEnd, Token::StructEnd) | (Token::StructVariantEnd, Token::StructVariantEnd) => {}
                (Token::UnitStruct {name: name_a}, Token::UnitStruct {name: name_b}) | (Token::NewtypeStruct {name: name_a}, Token::NewtypeStruct {name: name_b}) => {
                    if name_a != name_b {
                        return false;
                    }
                }
                (Token::UnitVariant {name: name_a, variant_index: variant_index_a, variant: variant_a}, Token::UnitVariant {name: name_b, variant_index: variant_index_b, variant: variant_b}) | (Token::NewtypeVariant {name: name_a, variant_index: variant_index_a, variant: variant_a}, Token::NewtypeVariant {name: name_b, variant_index: variant_index_b, variant: variant_b}) => {
                    if name_a != name_b || variant_index_a != variant_index_b || variant_a != variant_b {
                        return false;
                    }
                }
                (Token::Seq {len: len_a}, Token::Seq {len: len_b}) | (Token::Map {len: len_a}, Token::Map {len: len_b}) => {
                    if len_a != len_b {
                        return false;
                    }
                }
                (Token::Tuple {len: len_a}, Token::Tuple {len: len_b}) => {
                    if len_a != len_b {
                        return false;
                    }
                }
                (Token::TupleStruct {name: name_a, len: len_a}, Token::TupleStruct {name: name_b, len: len_b}) | (Token::Struct {name: name_a, len: len_a}, Token::Struct {name: name_b, len: len_b}) => {
                    if name_a != name_b || len_a != len_b {
                        return false;
                    }
                }
                (Token::TupleVariant {name: name_a, variant_index: variant_index_a, variant: variant_a, len: len_a}, Token::TupleVariant {name: name_b, variant_index: variant_index_b, variant: variant_b, len: len_b}) | (Token::StructVariant {name: name_a, variant_index: variant_index_a, variant: variant_a, len: len_a}, Token::StructVariant {name: name_b, variant_index: variant_index_b, variant: variant_b, len: len_b}) => {
                    if name_a != name_b || variant_index_a != variant_index_b || variant_a != variant_b || len_a != len_b {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
}
