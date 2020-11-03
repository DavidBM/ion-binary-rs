use crate::{IonHash, IonValue};
use sha2::Sha256;

#[test]
fn ion_hash_string() {
    let value = IonValue::String("Hola".to_string());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x39\xc4\xf3\x56\x39\xf5\xf2\xa5\x83\xc5\xe4\x39\x43\xc2\x06\x79\x92\xe5\x5e\xd2\xaa\x31\x90\x34\x28\x76\x56\x6c\xbf\xf6\x2e\xe0", &hash[..]);
}
