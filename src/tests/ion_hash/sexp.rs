use crate::{IonHash, IonValue};

#[test]
fn ion_hash_simple_sexp() {
    let value = IonValue::SExpr(vec![
        IonValue::String("this".into()),
        IonValue::String("is".into()),
        IonValue::String("a".into()),
        IonValue::String("test".into()),
        IonValue::Integer(42),
    ]);

    let hash = IonHash::default_digest(&value);

    assert_eq!(b"\x91\xd5\x62\xba\xa8\xa4\x7a\xf2\x0b\xfd\xde\x6f\xb1\x0c\xb8\xde\x34\xc2\xca\x2f\x38\x39\xb6\x7a\x13\x32\xe1\x6c\xf0\x08\x89\x75", &hash[..]);
}
