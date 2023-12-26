# Changelog

## Unreleased
### Changed
- `PartialEq` implementation for `Tokens` now avoids unnecessary iterator cloning when checking against `Unordered` `Token`s.

## 0.7.0 - 2023-12-23
### Added
- `Tokens` now implements `IntoIterator<Item = Token>`.
- The `token` module is now public, containing both `Token` (which is also exposed in the root module) and `Tokens`.
- Comparison with nested `Token::Unordered`s is now allowed and correctly handled.
- `token::IntoIter` type for iterating over the `Tokens` `struct`.
### Changed
- `Deserializer::build()` now takes the tokens as a parameter. These tokens can now be any type that implements `IntoIterator<Item = Token>`.
- `Tokens` is no longer exposed in the root module, instead being available at `token::Tokens`.
- The internals of `Tokens` are no longer public. `Tokens` can no longer be constructed by user code, and is now only returned by the `Serializer`.
- Comparison with a `Tokens` can now be done with any type that implements `IntoIterator<Item = &Token>`.
- `de::Builder::build()` now only requires `&self` instead of `&mut self`.
- `Error::ExpectedToken` variant has been replaced by `Error::ExpectedSeqEnd`, `Error::ExpectedTupleEnd`, `Error::ExpectedTupleStructEnd`, `Error::ExpectedTupleVariantEnd`, `Error::ExpectedMapEnd`, `Error::ExpectedStructEnd`, and `Error::ExpectedStructVariantEnd` variants.
### Removed
- `From<Token>` implementation for `serde::de::Unexpected`.
- `PartialEq` implementation for `Token`.
- `Display` implementation for `Token`.
- Dependency on `hashbrown` crate.

## 0.6.0 - 2023-11-19
### Changed
- Increased version of `hashbrown` dependency to `0.14.2`.
- Raised MSRV to `1.63.0`.

## 0.5.0 - 2023-05-16
### Added
- `Deserializer` can now be configured to allow (or disallow) zero-copy deserialization.

## 0.4.0 - 2023-04-06
### Added
- `ser::Error` now implements `PartialEq` and `Eq`.

## 0.3.0 - 2023-04-06
### Changed
- `Deserializer` now defaults to setting `self_describing` to `false`.

## 0.2.0 - 2023-01-16
### Added
- `SerializeStructAs` `enum` and accompanying `Builder::serialize_struct_as()` method for specifying whether `struct`s should be serialized as `serde` struct or sequence types.
- `SerializeStruct` type to provide a more specialized `serde::SerializeStruct` implementation.
### Changed
- `Deserializer::deserialize_struct` can now interpret both `Struct` and `Seq` `Token`s.
### Removed
- `CompoundSerializer` no longer implements `serde::SerializeStruct`.


## 0.1.0 - 2023-01-14
### Added
- `Token` and `Tokens` for representing serialized values.
- `Serializer`, `ser::Builder`, `ser::Error`, and `ser::CompoundSerializer` for testing serialization.
- `Deserializer`, `de::Builder`, and `de::Error` for testing deserialization.
