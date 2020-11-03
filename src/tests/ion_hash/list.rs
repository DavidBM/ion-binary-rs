use crate::{IonHash, IonValue};

#[test]
fn ion_hash_simple_list() {
    let value = IonValue::List(vec![
        IonValue::Integer(1),
        IonValue::Integer(2),
        IonValue::Integer(3),
        IonValue::Integer(-3),
        IonValue::Integer(-3354654),
    ]);

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x46\xf8\xa1\xd9\x02\xe3\x3e\x7e\x34\xec\xb6\x2e\xb7\xab\x90\x54\x69\x14\xa1\x53\xe1\x90\x96\xa5\x53\x13\x4a\x05\x01\xf6\xd3\xc3", &hash[..]);
}
