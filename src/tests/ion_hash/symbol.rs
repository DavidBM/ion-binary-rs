use crate::{IonHash, IonValue};
use sha2::Sha256;

#[test]
fn ion_hash_symbol() {
    let value = IonValue::Symbol("Hola".to_string());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x5b\xa7\xb1\xcd\xf4\xc6\x7b\xd4\x9e\x32\x25\x38\xad\x2a\x04\xaa\x11\xd7\xcd\xb1\x61\x49\xfe\x69\x9d\x78\xd2\xeb\xfb\xfc\xa8\x43", &hash[..]);
}
