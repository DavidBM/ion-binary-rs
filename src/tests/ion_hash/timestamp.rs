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
