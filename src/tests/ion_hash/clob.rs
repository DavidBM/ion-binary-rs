use crate::{IonHash, IonValue};
use sha2::Sha256;

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
