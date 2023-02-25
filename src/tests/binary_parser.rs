use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;
use num_bigint::{BigInt, BigUint};

#[test]
fn decode_value_null() {
    let ion_test = [0b_0000_1111u8];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_value_header(0),
        Ok(ValueHeader::new(0/*Null Value*/ + 15 /*Len for null Value*/).unwrap())
    );
}

#[test]
fn decode_varuint_one_byte() {
    let ion_test = [0b_1000_1000u8];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varuint(), Ok((BigUint::from(8u64), 1)));
}

#[test]
fn decode_varuint_two_byte_only_last_byte_significant() {
    let ion_test = [0b_0000_0000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varuint(), Ok((BigUint::from(8u64), 2)));
}

#[test]
fn decode_varuint_two_byte() {
    let ion_test = [0b_0001_0000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varuint(), Ok((BigUint::from(2056u64), 2)));
}

#[test]
fn decode_varuint_three_byte() {
    let ion_test = [0b_0001_0000, 0b_0000_1000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varuint(), Ok((BigUint::from(263176u64), 3)));
}

#[test]
fn decode_varuint_len_10() {
    let ion_test = [
        0b_0000_0001u8,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_1000_0000,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_varuint(),
        Ok((BigUint::from(9804371850199958528u64), 10))
    );
}

#[test]
fn decode_varint_one_byte_negative() {
    let ion_test = [0b_1100_1000u8];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(-8), 1)));
}

#[test]
fn decode_varint_one_byte_positive() {
    let ion_test = [0b_1000_1000u8];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(8), 1)));
}

#[test]
fn decode_varint_two_byte_only_last_byte_significant_negative() {
    let ion_test = [0b_0100_0000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(-8), 2)));
}

#[test]
fn decode_varint_two_byte_only_last_byte_significant_positive() {
    let ion_test = [0b_0000_0000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(8), 2)));
}

#[test]
fn decode_varint_two_byte_positive() {
    let ion_test = [0b_0001_0000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(2056), 2)));
}

#[test]
fn decode_varint_two_byte_negative() {
    let ion_test = [0b_0101_0000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(-2056), 2)));
}

#[test]
fn decode_varint_three_byte_positive() {
    let ion_test = [0b_0001_0000, 0b_0000_1000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(263176), 3)));
}

#[test]
fn decode_varint_three_byte_negative() {
    let ion_test = [0b_0101_0000, 0b_0000_1000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_varint(), Ok((BigInt::from(-263176), 3)));
}

#[test]
fn decode_varint_len_10_positive() {
    let ion_test = [
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_1000_1000,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_varint(),
        Ok((BigInt::from(580999813345182728i64), 9))
    );
}

#[test]
// Technically correct, but we don't handle this case (yet?)
fn decode_varint_len_10_max_positive() {
    let ion_test = [
        0b_0011_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_1111_1111,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_varint(),
        Ok((BigInt::from(4611686018427387903i64), 9))
    );
}

#[test]
// Technically correct, but we don't handle this case (yet?)
fn decode_varint_len_10_max_negative() {
    let ion_test = [
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_1111_1111,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_varint(),
        Ok((BigInt::from(-4611686018427387903i64), 9))
    );
}

#[test]
fn decode_uint_valid_len_8() {
    let ion_test = [
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_1000,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_uint(8), Ok(BigUint::from(8u64)));
}

#[test]
fn decode_uint_valid() {
    let ion_test = [0b_0000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_uint(1), Ok(BigUint::from(8u64)));
}

#[test]
fn decode_uint_valid_2() {
    let ion_test = [0b_0000_1000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_uint(2), Ok(BigUint::from(2184u64)));
}

#[test]
fn decode_uint_invalid_zero_len() {
    let ion_test = [0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_uint(0),
        Err(ParsingError::CannotReadZeroBytes)
    );
}

#[test]
fn decode_int_valid_len_8_positive() {
    let ion_test = [
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_1000,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(8), Ok(BigInt::from(8)));
}

#[test]
fn decode_int_valid_len_8_negative() {
    let ion_test = [
        0b_1000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_1000,
    ];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(8), Ok(BigInt::from(-8)));
}

#[test]
fn decode_int_valid_positive() {
    let ion_test = [0b_0000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(1), Ok(BigInt::from(8)));
}

#[test]
fn decode_int_valid_negative() {
    let ion_test = [0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(1), Ok(BigInt::from(-8)));
}

#[test]
fn decode_int_valid_2_positive() {
    let ion_test = [0b_0000_1000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(2), Ok(BigInt::from(2184)));
}

#[test]
fn decode_int_valid_2_negative() {
    let ion_test = [0b_1000_1000, 0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(2), Ok(BigInt::from(-2184)));
}

#[test]
fn decode_int_invalid_zero_len() {
    let ion_test = [0b_1000_1000];

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(lexer.consume_int(0), Err(ParsingError::CannotReadZeroBytes));
}

#[test]
fn decode_value_with_version_header() {
    let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83V";

    let mut lexer = IonBinaryParser::new(&ion_test[..]);

    assert_eq!(
        lexer.consume_value_header(0),
        Ok(ValueHeader::new((0xe << 4) /*Annotation*/ + 14 /*Long len*/).unwrap())
    );
}
