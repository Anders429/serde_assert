#![no_std]

extern crate alloc;

pub mod de;
pub mod ser;

mod token;

#[doc(inline)]
pub use ser::Serializer;
pub use token::{Token, Tokens};
