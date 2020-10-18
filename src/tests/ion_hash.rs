use crate::IonHash;

// The basic testing is taken from Amazons github implementation

// TODO: Implement all tests from https://github.com/amzn/ion-hash-test/blob/master/ion_hash_tests.ion

#[test]
fn ion_hash_general() {
    use sha2::Sha256;

    macro_rules! s {
        ($value:expr) => {
            IonHash::<Sha256>::from_bytes($value)
        };
    }

    // IonHash commutes
    assert_eq!(s!(b"1").dot(&s!(b"2")), s!(b"2").dot(&s!(b"1")));

    // Empty hashes
    assert_eq!(
        IonHash::<Sha256>::default().dot(&IonHash::default()),
        &mut IonHash::<Sha256>::default()
    );
    assert_eq!(s!(b"1").dot(&IonHash::default()), &mut s!(b"1"));
    assert_eq!(IonHash::default().dot(&s!(b"1")), &mut s!(b"1"));

    // An actual example, values checked against the Java implementation
    assert_eq!(
        s!(b"1").get()[..],
        b"\x6b\x86\xb2\x73\xff\x34\xfc\xe1\x9d\x6b\x80\x4e\xff\x5a\x3f\x57\x47\xad\xa4\xea\xa2\x2f\x1d\x49\xc0\x1e\x52\xdd\xb7\x87\x5b\x4b"[..]
    );
    assert_eq!(
        s!(b"2").get()[..],
        b"\xd4\x73\x5e\x3a\x26\x5e\x16\xee\xe0\x3f\x59\x71\x8b\x9b\x5d\x03\x01\x9c\x07\xd8\xb6\xc5\x1f\x90\xda\x3a\x66\x6e\xec\x13\xab\x35"[..]
    );
    assert_eq!(
        s!(b"1").dot(&s!(b"2")).get()[..],
        b"\x94\x0e\xd9\xab\xdd\xfb\x5e\xf2\x80\x04\x40\x85\x46\xbc\x50\x43\xcd\xa3\x91\x23\x2b\x6a\xfe\x07\x26\x7f\x9f\x8e\xd2\xb5\x00\xc9"[..]
    );
    assert_eq!(
        s!(b"2").dot(&s!(b"1")).get()[..],
        b"\x94\x0e\xd9\xab\xdd\xfb\x5e\xf2\x80\x04\x40\x85\x46\xbc\x50\x43\xcd\xa3\x91\x23\x2b\x6a\xfe\x07\x26\x7f\x9f\x8e\xd2\xb5\x00\xc9"[..]
    );
}
