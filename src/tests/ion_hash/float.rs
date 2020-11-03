use crate::{IonHash, IonValue};
use sha2::Sha256;

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
