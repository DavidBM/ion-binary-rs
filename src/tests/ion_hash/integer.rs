use crate::{IonHash, IonValue};
use sha2::Sha256;

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
