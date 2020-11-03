use crate::{IonHash, IonValue, NullIonValue};
use sha2::Sha256;

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
