use crate::hashmap;
use crate::{IonHash, IonValue, NullIonValue};
use bigdecimal::BigDecimal;
use sha2::Sha256;
use std::str::FromStr;

#[test]
fn ion_hash_general_1() {
    macro_rules! s {
        ($value:expr) => {
            IonHash::from_bytes::<Sha256>($value)
        };
    }

    // IonHash commutes
    assert_eq!(s!(b"1").dot(s!(b"2")), s!(b"2").dot(s!(b"1")));

    // Empty hashes
    assert_eq!(
        IonHash::default().dot(IonHash::default()),
        &mut IonHash::default()
    );
    assert_eq!(s!(b"1").dot(IonHash::default()), &mut s!(b"1"));
    assert_eq!(IonHash::default().dot(s!(b"1")), &mut s!(b"1"));

    // An actual example, values checked against the Java implementation
    assert_eq!(
        s!(b"1").get()[..],
        b"\x6b\x86\xb2\x73\xff\x34\xfc\xe1\x9d\x6b\x80\x4e\xff\x5a\x3f\x57\x47\xad\xa4\xea\xa2\x2f\x1d\x49\xc0\x1e\x52\xdd\xb7\x87\x5b\x4b"[..]
    );
    assert_eq!(
        s!(b"2").get()[..],
        b"\xd4\x73\x5e\x3a\x26\x5e\x16\xee\xe0\x3f\x59\x71\x8b\x9b\x5d\x03\x01\x9c\x07\xd8\xb6\xc5\x1f\x90\xda\x3a\x66\x6e\xec\x13\xab\x35"[..]
    );
    assert_eq!(
        s!(b"1").dot(s!(b"2")).get()[..],
        b"\x94\x0e\xd9\xab\xdd\xfb\x5e\xf2\x80\x04\x40\x85\x46\xbc\x50\x43\xcd\xa3\x91\x23\x2b\x6a\xfe\x07\x26\x7f\x9f\x8e\xd2\xb5\x00\xc9"[..]
    );
    assert_eq!(
        s!(b"2").dot(s!(b"1")).get()[..],
        b"\x94\x0e\xd9\xab\xdd\xfb\x5e\xf2\x80\x04\x40\x85\x46\xbc\x50\x43\xcd\xa3\x91\x23\x2b\x6a\xfe\x07\x26\x7f\x9f\x8e\xd2\xb5\x00\xc9"[..]
    );
}


// Hash for this test generated using 
// src/tests/ion_hash/reference_hash_impl/ion_hash_complex.ts
#[test]
fn ion_hash_general_2() {
    let list = IonValue::List(vec![
        IonValue::Integer(1),
        IonValue::Integer(2),
        IonValue::Integer(3),
        IonValue::Integer(-3),
        IonValue::Integer(-3354654),
    ]);

    let qldb_struct = IonValue::Struct(hashmap!(
        "Model".to_string() => IonValue::String("CLK 350".to_string()),
        "Type".to_string() => IonValue::String("Sedan".to_string()),
        "Color".to_string() => IonValue::String("White".to_string()),
        "VIN".to_string() => IonValue::String("1C4RJFAG0FC625797".to_string()),
        "Make".to_string() => IonValue::String("Mercedes".to_string()),
        "Year".to_string() => IonValue::Integer(2019)
    ));

    let long_struct = IonValue::Struct(hashmap!(
        "000021i".into() => IonValue::Integer(9),
        "012i".into() => IonValue::Integer(9),
        "01d".into() => IonValue::Integer(4),
        "01h".into() => IonValue::Integer(8),
        "11n".into() => IonValue::Float32(std::f32::NAN),
        "12l".into() => IonValue::Integer(12),
        "1d".into() => IonValue::Integer(4),
        "21l".into() => IonValue::Integer(12),
        "2h".into() => list,
        "aaa".into() => IonValue::Integer(1),
        "aak".into() => IonValue::Integer(11),
        "ae".into() => IonValue::Integer(5),
        "b".into() => qldb_struct,
        "bb".into() => IonValue::Integer(2),
        "cb".into() => IonValue::Integer(2),
        "c".into() => IonValue::Integer(3),
        "d".into() => IonValue::Null(NullIonValue::Clob),
        "9f".into() => IonValue::Integer(6),
        "09f".into() => IonValue::Decimal(BigDecimal::from_str("92407156491786485918754613897564897561387954629341564305176435762934857629384756024751649587623498561204576329654.1239476129586128957624351682956187465187324618724691845696216935").unwrap()),
        "g".into() => IonValue::Integer(7),
        "00h".into() => IonValue::Integer(8),
        "0h".into() => IonValue::Integer(8),
        "i".into() => IonValue::Integer(9),
        "j".into() => IonValue::Integer(10),
        "k".into() => IonValue::Null(NullIonValue::Float),
        "00001l".into() => IonValue::Integer(12),
        "00002l".into() => IonValue::Integer(12),
        "10000l".into() => IonValue::Integer(12),
        "l".into() => IonValue::Integer(12),
        "m".into() => IonValue::Integer(13),
        "n".into() => IonValue::Integer(14)
    ));

    let value = IonValue::Annotation(
        vec!["Annot 1".into(), "Annot 2".into(), "Annot 3".into()],
        Box::new(IonValue::Struct(hashmap!(
            "e".into() => IonValue::Integer(5),
            "a".into() => long_struct,
            "l".into() => IonValue::Integer(12),
            "b".into() => IonValue::Integer(2),
            "i".into() => IonValue::Integer(9),
            "n".into() => IonValue::Float32(123.12)
        ))),
    );

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\xeb\x22\x0f\xab\xcb\x85\x48\xb0\xe5\x7b\x6b\xfe\xed\xdb\x8d\xe8\x5d\x9b\x01\x75\xdd\x77\xb1\x15\x3b\xfc\xf6\x2d\x08\x9c\x61\x4b", &hash[..]);
}
