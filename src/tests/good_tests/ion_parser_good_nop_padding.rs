use crate::{hashmap, read_file_testsuite};
use crate::{
    ion_parser::IonParser, ion_parser_types::IonValue, IonParserError, NullIonValue, ParsingError,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[test]
fn nop_pad16_bytes() {
    let ion_blob = read_file_testsuite!("good/nopPad16Bytes");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::BinaryError(ParsingError::NoDataToRead)
    );
}

#[test]
fn nop_pad_inside_empty_struct_non_zero_symbol_id() {
    let ion_blob = read_file_testsuite!("good/nopPadInsideEmptyStructNonZeroSymbolId");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(HashMap::new())
    );
}

#[test]
fn nop_pad_inside_empty_struct_zero_symbol_id() {
    let ion_blob = read_file_testsuite!("good/nopPadInsideEmptyStructZeroSymbolId");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(HashMap::new())
    );
}

#[test]
fn nop_pad_inside_struct_with_nop_pad_then_value_non_zero_symbol_id() {
    let ion_blob =
        read_file_testsuite!("good/nopPadInsideStructWithNopPadThenValueNonZeroSymbolId");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!("name".to_string() => IonValue::Bool(true)))
    );
}

#[test]
fn nop_pad_inside_struct_with_nop_pad_then_value_zero_symbol_id() {
    let ion_blob = read_file_testsuite!("good/nopPadInsideStructWithNopPadThenValueZeroSymbolId");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!("name".to_string() => IonValue::Bool(true)))
    );
}

#[test]
fn nop_pad_inside_struct_with_value_then_nop_pad() {
    let ion_blob = read_file_testsuite!("good/nopPadInsideStructWithValueThenNopPad");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!("name".to_string() => IonValue::Bool(true)))
    );
}

#[test]
fn nop_pad_one_byte() {
    let ion_blob = read_file_testsuite!("good/nopPadOneByte");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::BinaryError(ParsingError::NoDataToRead)
    );
}

#[test]
fn value_between_nop_pads() {
    let ion_blob = read_file_testsuite!("good/valueBetweenNopPads");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Null)
    );
}

#[test]
fn value_followed_by_nop_pad() {
    let ion_blob = read_file_testsuite!("good/valueFollowedByNopPad");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Null)
    );
}

#[test]
fn value_preceded_by_nop_pad() {
    let ion_blob = read_file_testsuite!("good/valuePrecededByNopPad");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Null)
    );
}
