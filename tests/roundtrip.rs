use claims::{
    assert_ok,
    assert_ok_eq,
};
use serde::{
    Deserialize,
    Serialize,
};
use serde_assert::{
    Deserializer,
    Serializer,
};

#[test]
fn roundtrip() {
    let value = true;

    let serializer = Serializer::builder().build();
    let mut deserializer = Deserializer::builder()
        .tokens(assert_ok!(value.serialize(&serializer)))
        .build();

    assert_ok_eq!(bool::deserialize(&mut deserializer), value);
}
