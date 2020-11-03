use crate::{IonHash, IonValue};
use sha2::Sha256;

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
