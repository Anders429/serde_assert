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