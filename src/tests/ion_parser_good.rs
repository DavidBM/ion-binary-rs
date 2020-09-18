use crate::read_file_testsuite;
use bigdecimal::BigDecimal;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue, IonParserError, ParsingError};
use num_bigint::BigInt;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[test]
fn clob_with_del() {
    let ion_blob = read_file_testsuite!("good/clobWithDel");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(Vec::from([127]))
    );
}

#[test]
fn int_big_size256() {
    let ion_blob = read_file_testsuite!("good/intBigSize256");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"18173238162219679736857031944447898744767430095109316084451026048678348094928854458274167288816962557611640075817315237016025726423548207924331642028847993938530524659112028449811515920726159569583847554301932799584192974700038250645135419704389244690214111003505621818033044965879076306690914532152840279256440975668846810694285470204245958782248405612488959069641454132691581386219910938587286910894148564397155066367399697230287047229035630842240888106685623631032505806388903066971508775182055551847210338095961815021030725796281642316166745051164958432783938535334657296749823645911331793861360616240344479015948";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
        	BigInt::parse_bytes(expected_number_str, 10).unwrap())
    );
}

#[test]
fn int_big_size14() {
    let ion_blob = read_file_testsuite!("good/intBigSize14");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"2773783639172303802999334644566508";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
        	BigInt::parse_bytes(expected_number_str, 10).unwrap())
    );
}

#[test]
fn clob_with_non_ascii_character() {
    let ion_blob = read_file_testsuite!("good/clobWithNonAsciiCharacter");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(Vec::from([128]))
    );
}

#[test]
fn decimal_negative_one_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeOneDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"-1.0").unwrap())
    );
}

#[test]
fn decimal_negative_zero_dot() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeZeroDot");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"-0").unwrap())
    );
}

#[test]
fn decimal_negative_zero_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeZeroDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"-0.0").unwrap())
    );
}

#[test]
fn decimal_one_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalOneDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"1.0").unwrap())
    );
}

#[test]
fn decimal_zero_dot() {
    let ion_blob = read_file_testsuite!("good/decimalZeroDot");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"0.").unwrap())
    );
}

#[test]
fn empty_three_byte_nop_pad() {
    let ion_blob = read_file_testsuite!("good/emptyThreeByteNopPad");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::BinaryError(ParsingError::NoDataToRead)
    );
}

#[test]
fn float32() {
    let ion_blob = read_file_testsuite!("good/float32");

    let mut parser = IonParser::new(ion_blob);

    // 0000 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(0.0)
    );

    // 8000 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(-0.0)
    );

    // 4086 6666
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(4.2)
    );

    // c086 6666
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(-4.2)
    );

    // ff80 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(f32::NEG_INFINITY)
    );

    // 7f80 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(f32::INFINITY)
    );

    // ff7f ffff
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(-340282350000000000000000000000000000000.0)
    );

    // 7f7f ffff
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(340282350000000000000000000000000000000.0)
    );

    // 7fff ffff
    if let IonValue::Float32(number) = parser.consume_value().unwrap().0 {
        if !number.is_nan() {
            panic!("Result should be IonValue::Float32(f32::NAN)")
        }
    }

    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::BinaryError(ParsingError::NoDataToRead)
    );
}
