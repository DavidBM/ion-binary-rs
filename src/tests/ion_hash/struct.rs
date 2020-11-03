use crate::hashmap;
use crate::{IonHash, IonValue};

#[test]
fn ion_hash_3() {
    use crate::{IonHash, IonValue};
    use std::collections::HashMap;

    let mut ion_struct = HashMap::new();

    ion_struct.insert("Model".to_string(), IonValue::String("CLK 350".to_string()));
    ion_struct.insert("Type".to_string(), IonValue::String("Sedan".to_string()));
    ion_struct.insert("Color".to_string(), IonValue::String("White".to_string()));
    ion_struct.insert(
        "VIN".to_string(),
        IonValue::String("1C4RJFAG0FC625797".to_string()),
    );
    ion_struct.insert("Make".to_string(), IonValue::String("Mercedes".to_string()));
    ion_struct.insert("Year".to_string(), IonValue::Integer(2019));

    let ion_value = IonValue::Struct(ion_struct);

    let hash = IonHash::default_digest(&ion_value);

    println!("{:X?}", hash);

    assert_eq!(b"\x54\x06\x72\x50\x6e\x1f\x13\x2a\x5d\x38\x50\xff\x96\x00\xce\xc8\xce\x02\xeb\x95\x23\x93\x13\x71\x02\xec\x85\xfd\x15\xc8\xab\x05", &hash[..])
}

#[test]
fn ion_hash_simple_struct() {
    let value = IonValue::Struct(hashmap!(
        "e".into() => IonValue::Integer(5)
    ));

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x51\xdb\x1a\xe9\x86\x74\x3b\x61\x43\xa8\x37\x43\x67\x99\xb0\x9e\x73\xf1\x0b\x2b\xa8\x29\x9d\xe2\x8c\x19\x37\x73\x6f\xbb\x63\xb8", &hash[..]);
}

#[test]
fn ion_hash_long_struct() {
    let value = IonValue::Struct(hashmap!(
        "e".into() => IonValue::Integer(5),
        "a".into() => IonValue::Integer(1),
        "l".into() => IonValue::Integer(12),
        "b".into() => IonValue::Integer(2),
        "i".into() => IonValue::Integer(9),
        "n".into() => IonValue::Integer(14),
        "c".into() => IonValue::Integer(3),
        "j".into() => IonValue::Integer(10),
        "d".into() => IonValue::Integer(4),
        "f".into() => IonValue::Integer(6),
        "h".into() => IonValue::Integer(8),
        "k".into() => IonValue::Integer(11),
        "m".into() => IonValue::Integer(13),
        "g".into() => IonValue::Integer(7)
    ));

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x82\x8d\xd7\x95\x06\x30\x75\x60\xcb\x32\xaf\x18\xf0\x7e\x8b\x72\xc3\x16\x92\x52\x1c\x19\xcd\xa0\x6e\x38\x79\xf2\xb7\x9e\x2f\xaf", &hash[..]);
}

#[test]
fn ion_hash_long_long_struct() {
    let value = IonValue::Struct(hashmap!(
        "000021i".into() => IonValue::Integer(9),
        "012i".into() => IonValue::Integer(9),
        "01d".into() => IonValue::Integer(4),
        "01h".into() => IonValue::Integer(8),
        "11n".into() => IonValue::Integer(14),
        "12l".into() => IonValue::Integer(12),
        "1d".into() => IonValue::Integer(4),
        "21l".into() => IonValue::Integer(12),
        "2h".into() => IonValue::Integer(8),
        "aaa".into() => IonValue::Integer(1),
        "aak".into() => IonValue::Integer(11),
        "ae".into() => IonValue::Integer(5),
        "b".into() => IonValue::Integer(2),
        "bb".into() => IonValue::Integer(2),
        "cb".into() => IonValue::Integer(2),
        "c".into() => IonValue::Integer(3),
        "d".into() => IonValue::Integer(4),
        "9f".into() => IonValue::Integer(6),
        "09f".into() => IonValue::Integer(6),
        "g".into() => IonValue::Integer(7),
        "00h".into() => IonValue::Integer(8),
        "0h".into() => IonValue::Integer(8),
        "i".into() => IonValue::Integer(9),
        "j".into() => IonValue::Integer(10),
        "k".into() => IonValue::Integer(11),
        "00001l".into() => IonValue::Integer(12),
        "00002l".into() => IonValue::Integer(12),
        "10000l".into() => IonValue::Integer(12),
        "l".into() => IonValue::Integer(12),
        "m".into() => IonValue::Integer(13),
        "n".into() => IonValue::Integer(14)
    ));

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\xc5\xb0\xb2\x7c\x35\x54\xec\x01\x4f\x66\x49\x6c\x6a\x84\x7f\x3b\xaa\xfe\x0d\x23\xe5\x5b\x91\x1a\xd3\x1f\xb8\x71\xce\xd7\xf7\x8b", &hash[..]);
}
