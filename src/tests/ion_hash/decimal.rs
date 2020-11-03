use crate::{IonHash, IonValue};
use bigdecimal::BigDecimal;
use sha2::Sha256;
use std::str::FromStr;

#[test]
fn ion_hash_decimal_1() {
    let value = IonValue::Decimal(BigDecimal::from_str("12.34").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x6f\xd7\x1d\xbb\x0a\xc1\xce\xad\xd0\x6f\x7d\x18\x69\xc3\xa5\x1d\x37\x2f\xe1\xe2\xc4\x97\x42\x93\x7c\x06\xd4\x7c\x06\xe8\x1d\xa3", &hash[..]);
}

#[test]
fn ion_hash_decimal_2() {
    let value = IonValue::Decimal(BigDecimal::from_str("0").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\x2c\x58\x27\xb6\xd7\x7a\x31\x17\xa1\x55\xeb\x69\x9e\x26\x56\x85\x49\x2e\x19\x1b\x71\xbe\x6f\xf9\x36\x94\xff\x7f\xc9\xdd\xb6\x46", &hash[..]);
}

#[test]
fn ion_hash_decimal_3() {
    let value = IonValue::Decimal(BigDecimal::from_str("-0.0").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    // Technically speaking Ion can store -0.0 but BigDecimal cannot,
    // so we put this test in place hoping it will break and then we
    // will have a correct Ion implementation.
    assert_eq!(b"\x2c\x58\x27\xb6\xd7\x7a\x31\x17\xa1\x55\xeb\x69\x9e\x26\x56\x85\x49\x2e\x19\x1b\x71\xbe\x6f\xf9\x36\x94\xff\x7f\xc9\xdd\xb6\x46", &hash[..]);
}

#[test]
fn ion_hash_decimal_4() {
    let value = IonValue::Decimal(BigDecimal::from_str("-0.000000000000000000000000000000000000000000000000000000000123412356690101501598143987613957812309456159716591874596834").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    // Technically speaking Ion can store -0.0 but BigDecimal cannot,
    // so we put this test in place hoping it will break and then we
    // will have a correct Ion implementation.
    assert_eq!(b"\x02\xfa\xb2\x89\x5f\x19\xe9\x3f\x63\xcc\xbc\x0b\x89\x3d\xd9\xd0\x66\x6f\x36\x8c\xc8\xb4\x73\x6b\x23\xd7\xd2\xf5\xf7\x59\x45\x32", &hash[..]);
}

#[test]
fn ion_hash_decimal_5() {
    let value = IonValue::Decimal(BigDecimal::from_str("92407156491786485918754613897564897561387954629341564305176435762934857629384756024751649587623498561204576329654.1239476129586128957624351682956187465187324618724691845696216935").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    // Technically speaking Ion can store -0.0 but BigDecimal cannot,
    // so we put this test in place hoping it will break and then we
    // will have a correct Ion implementation.
    assert_eq!(b"\x4c\xff\x73\xd8\xad\x1e\xd0\x06\x2f\x5b\xd8\x16\x22\x35\x07\x4e\xa9\x2f\xba\xfc\xa9\x31\x9e\x01\x8f\x76\x9a\xb6\x65\x32\x6e\x50", &hash[..]);
}

#[test]
fn ion_hash_decimal_6() {
    let value = IonValue::Decimal(BigDecimal::from_str("-12.34").unwrap());

    let hash = IonHash::digest::<Sha256>(&value);

    println!("Resulting hash: {:X?}", hash);

    assert_eq!(b"\xd1\x2c\xb9\xe7\x42\x8d\x9d\x63\x61\x83\x02\x7f\x87\xbd\x75\xcc\x23\xe1\x03\xd5\x97\xec\xcc\x7f\xc0\x1b\x38\x32\xce\xe0\xaf\xbb", &hash[..]);
}
