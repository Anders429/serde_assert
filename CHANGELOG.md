# Changelog

## Unreleased
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
