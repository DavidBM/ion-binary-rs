use crate::hashmap;
use crate::{IonHash, IonValue, NullIonValue};

#[test]
fn ion_hash_annotation_1() {
    let value = IonValue::Annotation(
        vec!["Annot 1".into()],
        Box::new(IonValue::Null(NullIonValue::Null)),
    );

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x09\x44\x30\x5e\xf7\x77\xc6\x10\xdf\xf0\x8f\xc9\xd2\x04\xc9\xc8\xf0\xf7\x3b\x4b\x9a\xfe\xc6\xb9\x2e\xd3\x36\x8a\x1e\x05\xad\x7f", &hash[..]);
}

#[test]
fn ion_hash_annotation_2() {
    let value = IonValue::Annotation(
        vec!["Annot 1".into(), "Annot 2".into(), "Annot 3".into()],
        Box::new(IonValue::Struct(hashmap!(
            "e".into() => IonValue::Integer(5),
            "a".into() => IonValue::Integer(1),
            "l".into() => IonValue::Integer(12),
            "b".into() => IonValue::Integer(2),
            "i".into() => IonValue::Integer(9),
            "n".into() => IonValue::Integer(14)
        ))),
    );

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x6e\xbf\xeb\xda\xd9\xf4\xab\x09\xc3\x3b\x3e\xbb\xad\xc8\xbb\x77\x6c\x2e\xe2\x14\x5f\x00\xac\x71\x7c\xb9\x03\x72\xe7\x95\x60\x55", &hash[..]);
}
