use crate::hashmap;
use crate::{IonHash, IonValue};
use sha2::Sha256;

// The basic testing is taken from Amazons github implementation

// TODO: Implement all tests from https://github.com/amzn/ion-hash-test/blob/master/ion_hash_tests.ion

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



#[test]
fn ion_hash_general_simple_struct() {
    let value = IonValue::Struct(hashmap!(
        "e".into() => IonValue::Integer(5)
    ));

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x51\xdb\x1a\xe9\x86\x74\x3b\x61\x43\xa8\x37\x43\x67\x99\xb0\x9e\x73\xf1\x0b\x2b\xa8\x29\x9d\xe2\x8c\x19\x37\x73\x6f\xbb\x63\xb8", &hash[..]);
}

#[test]
fn ion_hash_general_long_struct() {
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
fn ion_hash_general_string() {
    let value = IonValue::String("Hola".to_string());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x39\xc4\xf3\x56\x39\xf5\xf2\xa5\x83\xc5\xe4\x39\x43\xc2\x06\x79\x92\xe5\x5e\xd2\xaa\x31\x90\x34\x28\x76\x56\x6c\xbf\xf6\x2e\xe0", &hash[..]);
}

#[test]
fn ion_hash_general_int() {
    let value = IonValue::Integer(1);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xf0\x89\xf6\x4c\xa7\x3b\x9b\x16\x0d\x33\xf1\x9b\x07\xf8\xd0\xc9\x7d\x4e\x8e\x42\x15\xc0\xb6\xb8\xb8\x36\xde\xdc\xfb\x65\x92\x9a", &hash[..]);
}

#[test]
fn ion_hash_general_negative_int() {
    let value = IonValue::Integer(-3);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x8a\xe5\x52\x93\xae\x65\x99\x36\xc7\x26\x9a\xb3\x59\x16\x0c\xfd\xde\xd0\x8d\x53\x10\xc0\xff\x1d\x4f\x77\xbb\xd9\x69\x9f\x48\xfc", &hash[..]);
}

#[test]
fn ion_hash_general_simple_list() {
    let value = IonValue::List(vec![
        IonValue::Integer(1),
        IonValue::Integer(2),
        IonValue::Integer(3),
        IonValue::Integer(-3),
        IonValue::Integer(-3354654),
    ]);

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x46\xf8\xa1\xd9\x02\xe3\x3e\x7e\x34\xec\xb6\x2e\xb7\xab\x90\x54\x69\x14\xa1\x53\xe1\x90\x96\xa5\x53\x13\x4a\x05\x01\xf6\xd3\xc3", &hash[..]);
}

#[test]
fn ion_hash_general_3() {
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
fn ion_hash_general_symbol() {
    let value = IonValue::Symbol("Hola".to_string());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x5b\xa7\xb1\xcd\xf4\xc6\x7b\xd4\x9e\x32\x25\x38\xad\x2a\x04\xaa\x11\xd7\xcd\xb1\x61\x49\xfe\x69\x9d\x78\xd2\xeb\xfb\xfc\xa8\x43", &hash[..]);
}

#[test]
fn ion_hash_general_datetime_1() {
    let value = IonValue::DateTime(chrono::DateTime::parse_from_rfc3339("2011-02-20T11:30:59.1-08:00").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xad\x6c\xea\xa0\xfb\x92\x31\x43\xf1\xa7\xcb\x88\x9a\x25\x1c\x1d\x82\xa1\x84\x5b\x6f\x90\x2d\xfa\x63\x85\xb2\x73\x74\x68\x86\xe7", &hash[..]);
}

#[test]
fn ion_hash_general_datetime_2() {
    let value = IonValue::DateTime(chrono::DateTime::parse_from_rfc3339("2234-11-01T23:59:59.999+03:45").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x7e\x68\x18\xa4\x10\x3e\x0f\x7a\xf8\x63\x1c\x92\x1a\x60\xa9\xb8\x35\x3f\x0c\x9b\x41\x69\xec\x8c\xee\xd9\xb4\xbe\xed\xaf\x23\x28", &hash[..]);
}
