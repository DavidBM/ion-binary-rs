use crate::{IonHash, IonValue};
use sha2::Sha256;

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
