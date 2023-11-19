# serde_assert

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Anders429/serde_assert/test.yaml?branch=master)](https://github.com/Anders429/serde_assert/actions/workflows/test.yaml)
[![codecov.io](https://img.shields.io/codecov/c/gh/Anders429/serde_assert)](https://codecov.io/gh/Anders429/serde_assert)
[![crates.io](https://img.shields.io/crates/v/serde_assert)](https://crates.io/crates/serde_assert)
[![docs.rs](https://docs.rs/serde_assert/badge.svg)](https://docs.rs/serde_assert)
[![MSRV](https://img.shields.io/badge/rustc-1.63.0+-yellow.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/crates/l/serde_assert)](#license)

Testing library for [`serde`](https://crates.io/crates/serde) [`Serialize`](https://docs.rs/serde/1.0.152/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/1.0.152/serde/trait.Deserialize.html) implementations.

This library provides a [`Serializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Serializer.html) and [`Deserializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Deserializer.html) to be used in writing unit tests to assert the behavior of manual [`Serialize`](https://docs.rs/serde/1.0.152/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/1.0.152/serde/trait.Deserialize.html) implementations, respectively. The implementation behavior can be verified by using [`Tokens`](https://docs.rs/serde_assert/latest/serde_assert/struct.Tokens.html) representing an arbitrary serialized state.

## Usage
The examples below use the [`claims`](https://crates.io/crates/claims) crate for convenient assertions.

### Testing Serialization
The [`Serializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Serializer.html) returns [`Tokens`](https://docs.rs/serde_assert/latest/serde_assert/struct.Tokens.html) representing the serialization of a value. The returned `Tokens` can be checked to be equal to an expected value.

```
use claims::assert_ok_eq;
use serde::Serialize;
use serde_assert::{
    Serializer,
    Token,
    Tokens,
};

let serializer = Serializer::builder().build();

assert_ok_eq!(true.serialize(&serializer), Tokens(vec![Token::Bool(true)]));
```

### Testing Deserialization
A [`Deserializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Deserializer.html) is constructed by providing [`Tokens`](https://docs.rs/serde_assert/latest/serde_assert/struct.Tokens.html) to be deserialized into a value.

```
use claims::assert_ok_eq;
use serde::Deserialize;
use serde_assert::{
    Deserializer,
    Token,
    Tokens,
};

let mut deserializer = Deserializer::builder()
    .tokens(Tokens(vec![Token::Bool(true)]))
    .build();

assert_ok_eq!(bool::deserialize(&mut deserializer), true);
```

## Comparison with [`serde_test`](https://crates.io/crates/serde_test)
This crate provides more flexibility than `serde_test` at the expense of more verbosity. While `serde_test` provides a small API of simple assertion macros, this crate will require you to call [`serialize()`](https://docs.rs/serde/latest/serde/trait.Serialize.html#tymethod.serialize) and [`deserialize()`](https://docs.rs/serde/latest/serde/trait.Deserialize.html#tymethod.deserialize) and assert yourself that the results are as expected.

While some users may find that the smaller API of `serde_test` is sufficient for their use-case, others will find that the flexibility of this crate makes testing more complicated `Serailize` and `Deserialize` implementations easier. Among other things, this crate's API provides these advantages:

- Direct access to the [`Serializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Serializer.html) and [`Deserializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Deserializer.html), allowing use of all parts of the `serde` `Serializer` and `Deserializer` APIs, such as deserializing types that implement [`DeserializeSeed`](https://docs.rs/serde/latest/serde/de/trait.DeserializeSeed.html).
- Customization of [`Serializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Serializer.html)s and [`Deserializer`](https://docs.rs/serde_assert/latest/serde_assert/struct.Deserializer.html)s, allowing configuration of things like human-readability, whether the `Deserializer` should interpret [`Tokens`](https://docs.rs/serde_assert/latest/serde_assert/struct.Tokens.html) as self-describing, and zero-copy deserialization.
- Sophisticated comparison of serialized [`Tokens`](https://docs.rs/serde_assert/latest/serde_assert/struct.Tokens.html), including allowing testing of types whose serialized form can include items in arbitrary order, such as when serializing a [`HashSet`](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html).

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.63.0` and up.

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/serde_assert/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/serde_assert/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
