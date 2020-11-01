use crate::hashmap;
use crate::{IonHash, IonValue, NullIonValue};
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

// STRING

#[test]
fn ion_hash_string() {
    let value = IonValue::String("Hola".to_string());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x39\xc4\xf3\x56\x39\xf5\xf2\xa5\x83\xc5\xe4\x39\x43\xc2\x06\x79\x92\xe5\x5e\xd2\xaa\x31\x90\x34\x28\x76\x56\x6c\xbf\xf6\x2e\xe0", &hash[..]);
}

// INTEGER

#[test]
fn ion_hash_int() {
    let value = IonValue::Integer(1);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xf0\x89\xf6\x4c\xa7\x3b\x9b\x16\x0d\x33\xf1\x9b\x07\xf8\xd0\xc9\x7d\x4e\x8e\x42\x15\xc0\xb6\xb8\xb8\x36\xde\xdc\xfb\x65\x92\x9a", &hash[..]);
}

#[test]
fn ion_hash_negative_int() {
    let value = IonValue::Integer(-3);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x8a\xe5\x52\x93\xae\x65\x99\x36\xc7\x26\x9a\xb3\x59\x16\x0c\xfd\xde\xd0\x8d\x53\x10\xc0\xff\x1d\x4f\x77\xbb\xd9\x69\x9f\x48\xfc", &hash[..]);
}

// LIST

#[test]
fn ion_hash_simple_list() {
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

// STRUCT

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

// SYMBOL

#[test]
fn ion_hash_symbol() {
    let value = IonValue::Symbol("Hola".to_string());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x5b\xa7\xb1\xcd\xf4\xc6\x7b\xd4\x9e\x32\x25\x38\xad\x2a\x04\xaa\x11\xd7\xcd\xb1\x61\x49\xfe\x69\x9d\x78\xd2\xeb\xfb\xfc\xa8\x43", &hash[..]);
}

// TIMESTAMP

#[test]
fn ion_hash_datetime_1() {
    let value = IonValue::DateTime(
        chrono::DateTime::parse_from_rfc3339("2011-02-20T11:30:59.1-08:00").unwrap(),
    );

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xad\x6c\xea\xa0\xfb\x92\x31\x43\xf1\xa7\xcb\x88\x9a\x25\x1c\x1d\x82\xa1\x84\x5b\x6f\x90\x2d\xfa\x63\x85\xb2\x73\x74\x68\x86\xe7", &hash[..]);
}

#[test]
fn ion_hash_datetime_2() {
    let value = IonValue::DateTime(
        chrono::DateTime::parse_from_rfc3339("2234-11-01T23:59:59.999+03:45").unwrap(),
    );

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x7e\x68\x18\xa4\x10\x3e\x0f\x7a\xf8\x63\x1c\x92\x1a\x60\xa9\xb8\x35\x3f\x0c\x9b\x41\x69\xec\x8c\xee\xd9\xb4\xbe\xed\xaf\x23\x28", &hash[..]);
}

// BOOL 

#[test]
fn ion_hash_bool_true() {
    let value = IonValue::Bool(true);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xce\xe5\x44\x99\xd5\xf3\x62\xb2\x72\xfb\xd8\xee\x64\x80\xff\x54\x7a\x6d\xc4\xe2\xd9\xe1\x27\x33\x45\x9f\x82\x0e\x70\x30\x50\x17", &hash[..]);
}

#[test]
fn ion_hash_bool_false() {
    let value = IonValue::Bool(false);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x85\xed\x3a\xb0\xdc\xf0\x03\xe3\x2c\x98\x71\xc0\x22\x0f\xf7\x9f\xe2\xa1\xd5\xf0\xc9\x51\x01\x67\x07\x72\xd7\xde\xa9\x46\xa2\x67", &hash[..]);
}

// NULL

#[test]
fn ion_hash_bool_null() {
    let value = IonValue::Null(NullIonValue::Null);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x0f\xb0\x6b\x61\x83\xc2\x13\x79\x52\x9f\xdd\x45\xd6\xaf\x4a\xba\x73\x1a\xc6\xf0\x81\xef\x9e\x6c\x1c\x94\xb1\xfb\x26\x17\x73\x04", &hash[..]);
}

#[test]
fn ion_hash_bool_null_blob() {
    let value = IonValue::Null(NullIonValue::Blob);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x95\xc1\x6c\x6e\x85\xb0\x7e\x81\x3a\x62\xc4\x5a\xb9\x7f\xc2\xa8\x20\xe4\x20\xd9\x85\xc7\x57\x12\x61\xf6\x8f\x73\xaa\xb9\xdd\xa0", &hash[..]);
}

#[test]
fn ion_hash_bool_null_bool() {
    let value = IonValue::Null(NullIonValue::Bool);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xdd\xcd\x5e\x1d\x76\x28\x9a\xb8\x5d\xcb\x7f\x7a\x10\x5d\x67\x3f\xea\x25\xb5\x67\x39\x3f\xd1\x3d\xdc\x83\x7b\x19\x5f\x3a\xa9\xa6", &hash[..]);
}

#[test]
fn ion_hash_bool_null_clob() {
    let value = IonValue::Null(NullIonValue::Clob);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x16\xc1\x55\x03\xb1\xf0\xbc\x20\x04\xf7\xea\x79\x5a\x52\x85\x0a\x1f\x66\x81\x90\x69\x8c\x8d\x76\x16\xf0\x3d\x9e\xf1\x26\xd0\xcb", &hash[..]);
}

#[test]
fn ion_hash_bool_null_decimal() {
    let value = IonValue::Null(NullIonValue::Decimal);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xbe\x98\x38\xf3\xbc\x1e\x18\xb6\xaf\xb2\xb7\xa0\x80\x8e\xf5\xef\xb3\xb5\xee\xf0\xd6\x8d\x6e\x72\xaf\xcd\x4a\xea\x4f\xbe\xc5\x92", &hash[..]);
}

#[test]
fn ion_hash_bool_null_float() {
    let value = IonValue::Null(NullIonValue::Float);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xad\x79\xf0\x57\x0f\x33\x77\xc1\xae\x14\x60\x83\x57\x1c\xb7\x38\xa5\xf9\x5f\x73\xbd\x4e\x09\x1c\xc6\xc5\x14\x97\xa5\x0c\x97\xd4", &hash[..]);
}

#[test]
fn ion_hash_bool_null_integer() {
    let value = IonValue::Null(NullIonValue::Integer);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xc8\x05\x35\x73\x4f\xef\xff\xeb\xc4\x49\xc3\xef\x70\x82\x7e\xa6\x95\x45\x51\xa1\xb2\x5c\x39\x6b\xc3\x0a\xb0\x2d\x64\x15\xb1\x99", &hash[..]);
}

#[test]
fn ion_hash_bool_null_list() {
    let value = IonValue::Null(NullIonValue::List);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xdc\x36\xa1\x86\xe5\x8e\xfd\xfb\xf7\x43\x07\x08\x3f\x1f\xee\xf1\xdc\xe7\xc9\x8b\xf5\x4c\xa6\x25\x1c\x67\x7c\x81\x95\xb0\xae\x85", &hash[..]);
}

#[test]
fn ion_hash_bool_null_sexp() {
    let value = IonValue::Null(NullIonValue::SExpr);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x42\xef\x00\xce\xba\xdf\xff\xa7\x6d\xfd\x07\x3c\x09\x2e\xd4\x2e\xdc\x0b\x8a\xa4\x36\x90\xa6\x6d\x10\x2a\xee\xec\x2f\x94\x72\x44", &hash[..]);
}

#[test]
fn ion_hash_bool_null_string() {
    let value = IonValue::Null(NullIonValue::String);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xa5\x4a\xbf\xe9\xfc\x39\xf5\xab\x44\xae\x80\x41\x2a\xd4\xb9\xd7\x00\xf9\x92\x05\xdc\x20\x08\x15\x3a\x1d\x28\x9e\xf6\xd7\x30\xfc", &hash[..]);
}

#[test]
fn ion_hash_bool_null_struct() {
    let value = IonValue::Null(NullIonValue::Struct);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x87\x45\x1a\x2e\x7d\x09\xb5\x04\xb2\xd8\x32\x23\x92\x80\x95\xd8\xfc\x15\xdb\xf0\x53\xf1\x6b\xfa\xf3\x0f\xa0\x57\x9c\x37\x41\x55", &hash[..]);
}

#[test]
fn ion_hash_bool_null_symbol() {
    let value = IonValue::Null(NullIonValue::Symbol);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xbb\x0e\xc1\x82\x69\xa9\x81\xce\x3c\xef\xbd\xa0\xda\xa8\xfe\x6f\x57\x14\x37\x38\x32\x4e\xaf\x2a\x82\xd9\xdf\x4f\x0e\x54\x28\x7e", &hash[..]);
}

#[test]
fn ion_hash_bool_null_datetime() {
    let value = IonValue::Null(NullIonValue::DateTime);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x8a\x06\x02\x72\xce\x85\x30\x35\x7e\x7a\x7a\xef\x7b\xd8\x40\xc0\x60\x32\xf6\xba\xe8\x50\xb7\xa3\x64\x69\x1c\xc1\xa5\x12\xdd\x30", &hash[..]);
}
