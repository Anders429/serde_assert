# Changelog

## Unreleased

## 0.6.0 - 2023-11-19
### Changed
- Increased version of `hashbrown` dependency to `0.14.2`.
- Raised MSRV to `1.64.0`.

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
- `SerializeStructAs``enum` and accompanying `Builder::serialize_struct_as()` method for specifying whether `struct`s should be serialized as `serde` struct or sequence types.
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
