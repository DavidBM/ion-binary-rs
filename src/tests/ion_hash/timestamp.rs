use std::collections::HashMap;
use crate::{IonHash, IonValue};
use sha2::Sha256;

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

#[test]
fn ion_hash_datetime_3() {
    let value = IonValue::DateTime(
        chrono::DateTime::parse_from_rfc3339("2011-01-01T00:00:00+00:00").unwrap(),
    );

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x0c\xa1\xd8\xc2\x14\xf7\xf0\x26\xc9\xb8\xe8\x7f\x61\xf6\xf5\x3d\xaf\xfc\x38\x51\x2e\x6c\xbf\x05\xe6\x45\x5b\xe1\x3a\xa6\xda\x4a", &hash[..]);
}

#[test]
fn ion_hash_datetime_4() {
    let value = IonValue::DateTime(
        // In this case, this is the equivalent to encode in JS
        // 2011-02-20T11:30:59.1-08:00 without following zeros 
        // in the seconds decimals places, as in JS is not the same
        // .100 than .1 because the Ion system. Here we never 
        // represent the DateTime in string, so we don't know how
        // many following zeros has the decimal. This implementation
        // removes them all and assumes the minimum precision for the
        // number to be represented. 
        chrono::DateTime::parse_from_rfc3339("2011-02-20T11:30:59.100-08:00").unwrap(),
    );

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xad\x6c\xea\xa0\xfb\x92\x31\x43\xf1\xa7\xcb\x88\x9a\x25\x1c\x1d\x82\xa1\x84\x5b\x6f\x90\x2d\xfa\x63\x85\xb2\x73\x74\x68\x86\xe7", &hash[..]);
}

#[test]
fn ion_hash_datetimes_in_struct() {
    let mut map = HashMap::new();
    map.insert(
        "2011-01-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-01-01T00:00:00+00:00").unwrap(),
        ),
    );

    let value = IonValue::Struct(map);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xb3\x85\xbc\x1d\x81\xa8\x95\x76\xc2\xd8\x03\xbc\xe0\x62\x64\x48\x20\x3b\x6d\x0d\xb2\x0f\x70\xc2\xa7\x5c\xc6\x93\x1f\x3c\x76\x5f", &hash[..]);
}

#[test]
fn ion_hash_datetimes_in_struct_2() {
    let mut map = HashMap::new();
    map.insert(
        "2011-01-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-01-01T00:00:00+00:00").unwrap(),
        ),
    );
    map.insert(
        "2011-02-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-01T00:00:00+00:00").unwrap(),
        ),
    );

    let value = IonValue::Struct(map);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x2e\x8b\x56\x1b\x8a\xd3\x2c\x95\x5b\x2e\xee\xc4\x4d\x71\xfa\x64\x59\xaf\x00\x0a\x3e\x52\xd0\xaf\x8a\xae\x31\x6d\x9e\x6e\x2d\xa0", &hash[..]);
}

#[test]
fn ion_hash_datetimes_in_struct_3() {
    let mut map = HashMap::new();
    map.insert(
        "2011-01-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-01-01T00:00:00+00:00").unwrap(),
        ),
    );
    map.insert(
        "2011-02-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-01T00:00:00+00:00").unwrap(),
        ),
    );
    map.insert(
        "2011-02-20T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-20T00:00:00+00:00").unwrap(),
        ),
    );

    let value = IonValue::Struct(map);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xf8\x0c\x1c\xe9\xd1\x28\x7d\x5d\x72\x1e\x09\x99\x81\x37\x08\x7e\xb0\x21\x45\xfc\x9d\xbe\xeb\x43\xc0\xc5\xa2\xd9\xcc\xd7\xb4\xfa", &hash[..]);
}

#[test]
fn ion_hash_datetimes_in_struct_4() {
    let mut map = HashMap::new();
    map.insert(
        "2011-01-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-01-01T00:00:00+00:00").unwrap(),
        ),
    );
    map.insert(
        "2011-02-01T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-01T00:00:00+00:00").unwrap(),
        ),
    );
    map.insert(
        "2011-02-20T00:00:00+00:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-20T00:00:00+00:00").unwrap(),
        ),
    );
    map.insert(
        "2011-02-20T11:30:59.100-08:00".into(),
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-20T11:30:59.100-08:00").unwrap(),
        ),
    );
    let value = IonValue::Struct(map);

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x59\x5f\x83\xe8\xfa\xb5\x45\xd7\xd4\xa8\x0d\x05\x2d\x25\x63\x92\xae\x4b\xaa\xcd\x89\x49\x4a\x3c\x25\x28\xb9\xea\xed\xe2\xd7\x15", &hash[..]);
}

