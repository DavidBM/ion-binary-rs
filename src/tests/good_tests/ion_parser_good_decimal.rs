use crate::read_file_testsuite;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue};
use bigdecimal::BigDecimal;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[test]
fn decimal_negative_one_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeOneDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str("-1.0").unwrap())
    );
}

#[test]
fn decimal_negative_zero_dot() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeZeroDot");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str("-0").unwrap())
    );
}

#[test]
fn decimal_negative_zero_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeZeroDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str("-0.0").unwrap())
    );
}

#[test]
fn decimal_one_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalOneDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str("1.0").unwrap())
    );
}

#[test]
fn decimal_zero_dot() {
    let ion_blob = read_file_testsuite!("good/decimalZeroDot");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str("0.").unwrap())
    );
}
