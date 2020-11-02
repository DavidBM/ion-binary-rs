use crate::hashmap;
use crate::{IonHash, IonValue, NullIonValue};
use sha2::Sha256;
use bigdecimal::BigDecimal;
use std::str::FromStr;

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

// SEXP

#[test]
fn ion_hash_simple_sexp() {
    let value = IonValue::SExpr(vec![
        IonValue::String("this".into()),
        IonValue::String("is".into()),
        IonValue::String("a".into()),
        IonValue::String("test".into()),
        IonValue::Integer(42),
    ]);

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x91\xd5\x62\xba\xa8\xa4\x7a\xf2\x0b\xfd\xde\x6f\xb1\x0c\xb8\xde\x34\xc2\xca\x2f\x38\x39\xb6\x7a\x13\x32\xe1\x6c\xf0\x08\x89\x75", &hash[..]);
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

// FLOAT 64

#[test]
fn ion_hash_float64_1() {
    let value = IonValue::Float64(123.123);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x77\x97\x4d\x9d\x8f\x47\xb7\xca\xfa\x7e\xfb\x82\x12\xa7\xa6\xb4\x84\x08\xd5\xd3\x4f\xbf\x65\xbb\x51\xbd\xa1\xc5\x21\x95\x08\x83", &hash[..]);
}

#[test]
fn ion_hash_float64_2() {
    let value = IonValue::Float64(std::f64::INFINITY);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x99\x85\xea\xd9\x83\xf1\x93\x9d\x38\x1b\x22\xe3\x06\x78\xe0\x9c\x74\x36\x9b\x77\x07\xdc\x3f\x17\xc2\xfd\x5e\x6c\xe4\x24\xd4\x63", &hash[..]);
}

#[test]
fn ion_hash_float64_3() {
    let value = IonValue::Float64(std::f64::NEG_INFINITY);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x97\x70\x38\x64\xea\x12\x9b\x09\x8b\x5f\x85\xab\xe1\x86\x1c\xab\x5a\x94\xfe\xbc\x1a\xd4\x0e\x93\x7e\x15\xa0\x47\x7b\x51\xc3\x68", &hash[..]);
}

#[test]
fn ion_hash_float64_4() {
    let value = IonValue::Float64(std::f64::NAN);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xed\x86\x50\x17\xdd\x3f\x6d\x06\xe5\x7b\xab\xc0\x2e\xe8\xc2\xd0\x15\x38\xad\xb6\x31\x6c\xd4\x71\x63\xf4\xd8\x13\xda\x2b\x98\x9d", &hash[..]);
}

#[test]
fn ion_hash_float64_5() {
    let value = IonValue::Float64(-123.123);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xec\x5e\xae\x54\x67\x3a\xc2\xcb\x3c\xc7\x5c\xde\x4d\x9f\xec\x1a\xde\xc1\x08\x40\x97\x6a\xd9\x79\x96\x64\x4d\x76\x20\x90\xc6\x51", &hash[..]);
}

// FLOAT 32

#[test]
fn ion_hash_float32_1() {
    let value = IonValue::Float32(123.12);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x09\xf5\x80\xf4\x55\x26\x3f\xf7\x48\xe3\xb9\xf5\x78\x5a\x5e\x6e\xeb\xd8\x72\xd5\xcc\xea\x4e\x85\xde\x20\x58\x03\x24\xb8\xfb\x99", &hash[..]);
}

#[test]
fn ion_hash_float32_2() {
    let value = IonValue::Float32(std::f32::INFINITY);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x99\x85\xea\xd9\x83\xf1\x93\x9d\x38\x1b\x22\xe3\x06\x78\xe0\x9c\x74\x36\x9b\x77\x07\xdc\x3f\x17\xc2\xfd\x5e\x6c\xe4\x24\xd4\x63", &hash[..]);
}

#[test]
fn ion_hash_float32_3() {
    let value = IonValue::Float32(std::f32::NEG_INFINITY);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x97\x70\x38\x64\xea\x12\x9b\x09\x8b\x5f\x85\xab\xe1\x86\x1c\xab\x5a\x94\xfe\xbc\x1a\xd4\x0e\x93\x7e\x15\xa0\x47\x7b\x51\xc3\x68", &hash[..]);
}

#[test]
fn ion_hash_float32_4() {
    let value = IonValue::Float32(std::f32::NAN);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xed\x86\x50\x17\xdd\x3f\x6d\x06\xe5\x7b\xab\xc0\x2e\xe8\xc2\xd0\x15\x38\xad\xb6\x31\x6c\xd4\x71\x63\xf4\xd8\x13\xda\x2b\x98\x9d", &hash[..]);
}

#[test]
fn ion_hash_float32_5() {
    let value = IonValue::Float32(-123.12);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xb6\x73\x52\x88\xaf\xba\x1c\x84\xb5\xad\xd0\x63\xd1\x77\x91\x13\x26\xd5\x92\x91\x41\x22\x73\x80\x92\x78\x08\x32\xea\xea\xd1\xc5", &hash[..]);
}

// DECIMAL

#[test]
fn ion_hash_decimal_1() {
    let value = IonValue::Decimal(BigDecimal::from_str("12.34").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x6f\xd7\x1d\xbb\x0a\xc1\xce\xad\xd0\x6f\x7d\x18\x69\xc3\xa5\x1d\x37\x2f\xe1\xe2\xc4\x97\x42\x93\x7c\x06\xd4\x7c\x06\xe8\x1d\xa3", &hash[..]);
}

#[test]
fn ion_hash_decimal_2() {
    let value = IonValue::Decimal(BigDecimal::from_str("0").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x2c\x58\x27\xb6\xd7\x7a\x31\x17\xa1\x55\xeb\x69\x9e\x26\x56\x85\x49\x2e\x19\x1b\x71\xbe\x6f\xf9\x36\x94\xff\x7f\xc9\xdd\xb6\x46", &hash[..]);
}

#[test]
fn ion_hash_decimal_3() {
    let value = IonValue::Decimal(BigDecimal::from_str("-0.0").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    // Technically speaking Ion can store -0.0 but BigDecimal cannot, 
    // so we put this test in place hoping it will break and then we
    // will have a correct Ion implementation.
    assert_eq!(b"\x2c\x58\x27\xb6\xd7\x7a\x31\x17\xa1\x55\xeb\x69\x9e\x26\x56\x85\x49\x2e\x19\x1b\x71\xbe\x6f\xf9\x36\x94\xff\x7f\xc9\xdd\xb6\x46", &hash[..]);
}

#[test]
fn ion_hash_decimal_4() {
    let value = IonValue::Decimal(BigDecimal::from_str("-0.000000000000000000000000000000000000000000000000000000000123412356690101501598143987613957812309456159716591874596834").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    // Technically speaking Ion can store -0.0 but BigDecimal cannot, 
    // so we put this test in place hoping it will break and then we
    // will have a correct Ion implementation.
    assert_eq!(b"\x02\xfa\xb2\x89\x5f\x19\xe9\x3f\x63\xcc\xbc\x0b\x89\x3d\xd9\xd0\x66\x6f\x36\x8c\xc8\xb4\x73\x6b\x23\xd7\xd2\xf5\xf7\x59\x45\x32", &hash[..]);
}

#[test]
fn ion_hash_decimal_5() {
    let value = IonValue::Decimal(BigDecimal::from_str("92407156491786485918754613897564897561387954629341564305176435762934857629384756024751649587623498561204576329654.1239476129586128957624351682956187465187324618724691845696216935").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    // Technically speaking Ion can store -0.0 but BigDecimal cannot, 
    // so we put this test in place hoping it will break and then we
    // will have a correct Ion implementation.
    assert_eq!(b"\x4c\xff\x73\xd8\xad\x1e\xd0\x06\x2f\x5b\xd8\x16\x22\x35\x07\x4e\xa9\x2f\xba\xfc\xa9\x31\x9e\x01\x8f\x76\x9a\xb6\x65\x32\x6e\x50", &hash[..]);
}

#[test]
fn ion_hash_decimal_6() {
    let value = IonValue::Decimal(BigDecimal::from_str("-12.34").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xd1\x2c\xb9\xe7\x42\x8d\x9d\x63\x61\x83\x02\x7f\x87\xbd\x75\xcc\x23\xe1\x03\xd5\x97\xec\xcc\x7f\xc0\x1b\x38\x32\xce\xe0\xaf\xbb", &hash[..]);
}

// CLOB

#[test]
fn ion_hash_clob_1() {
    let value = IonValue::Clob(b"clobtest".to_vec());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x4a\x90\xec\x6c\xf2\x60\xce\x02\xee\x03\xd1\x09\x36\x37\x52\x77\x54\x6c\xd1\x6b\xa4\x95\x24\xac\xa4\x7e\xb5\xbe\x38\xe8\xd4\xba", &hash[..]);
}

#[test]
fn ion_hash_clob_2() {
    let value = IonValue::Clob(vec![]);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x32\x07\x70\xb9\xc1\x2a\x79\x9d\x54\x0d\x5f\x8b\x36\xb6\x5e\x84\xbe\x6d\x1f\xd8\x6f\xcc\x49\x5b\x46\x84\xdc\xf1\x17\x58\xb6\x6f", &hash[..]);
}

// BLOB

#[test]
fn ion_hash_blob_1() {
    let value = IonValue::Blob(b"clobtest".to_vec());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x41\x7e\x01\x85\x38\xa7\x3e\xa2\xeb\x47\xcf\xa4\x66\xbf\x28\xf0\xe3\x28\xb8\x8f\x7a\xdf\x0a\xfc\x2a\x5f\xfd\xa5\x55\x5a\xe0\xe1", &hash[..]);
}

#[test]
fn ion_hash_blob_2() {
    let value = IonValue::Blob(vec![]);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x40\x39\x1b\x6d\xb6\x0d\xb7\xae\xf0\x2b\xab\xee\x5b\x77\xd9\x33\xe3\x64\xe3\xf2\xdd\xc1\xdf\xa3\xf0\x97\xf0\xc6\xf9\x08\x5b\x34", &hash[..]);
}

// ANNOTATION

#[test]
fn ion_hash_annotation_1() {
    let value = IonValue::Annotation(vec![
        "Annot 1".into()
    ], Box::new(IonValue::Null(NullIonValue::Null)));

    let hash = IonHash::default_digest(&value);

    // TODO: The JS implementation doesn't support annotations, so no test for now.
    assert_eq!(b"\x09\x44\x30\x5e\xf7\x77\xc6\x10\xdf\xf0\x8f\xc9\xd2\x04\xc9\xc8\xf0\xf7\x3b\x4b\x9a\xfe\xc6\xb9\x2e\xd3\x36\x8a\x1e\x05\xad\x7f", &hash[..]);
}

#[test]
fn ion_hash_annotation_2() {
    let value = IonValue::Annotation(vec![
        "Annot 1".into(),
        "Annot 2".into(),
        "Annot 3".into()
    ], Box::new(IonValue::Struct(hashmap!(
        "e".into() => IonValue::Integer(5),
        "a".into() => IonValue::Integer(1),
        "l".into() => IonValue::Integer(12),
        "b".into() => IonValue::Integer(2),
        "i".into() => IonValue::Integer(9),
        "n".into() => IonValue::Integer(14)
    ))));

    let hash = IonHash::default_digest(&value);

    // TODO: The JS implementation doesn't support annotations, so no test for now.
    assert_eq!(b"\x6e\xbf\xeb\xda\xd9\xf4\xab\x09\xc3\x3b\x3e\xbb\xad\xc8\xbb\x77\x6c\x2e\xe2\x14\x5f\x00\xac\x71\x7c\xb9\x03\x72\xe7\x95\x60\x55", &hash[..]);
}
