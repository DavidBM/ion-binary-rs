use crate::read_file_testsuite;
use crate::IonParserError;
use crate::ParsingError;
use crate::{binary_parser_types::ValueLength, ion_parser::IonParser};
use bigdecimal::BigDecimal;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[test]
fn bad_magic_1015() {
    // let ion_element = read_file_testsuite!("bad/badMagic1015");
    // Not implemented due to we don't want to fail if there's a file with no header.
}

#[test]
fn bad_magic_e00100e0() {
    let ion_element = read_file_testsuite!("bad/badMagicE00100E0");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::BadFormedVersionHeader);
    assert_eq!(expected, value);
}

#[test]
fn blob_len_too_large() {
    let ion_element = read_file_testsuite!("bad/blobLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(14));
    assert_eq!(expected, value);
}

#[test]
fn bool_with_invalid_length_1() {
    let ion_element = read_file_testsuite!("bad/boolWithInvalidLength_1");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(3));
    assert_eq!(expected, value);
}

#[test]
fn bool_with_invalid_length_2() {
    let ion_element = read_file_testsuite!("bad/boolWithInvalidLength_2");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::LongLength);
    assert_eq!(expected, value);
}

#[test]
fn clob_len_too_large() {
    let ion_element = read_file_testsuite!("bad/clobLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(16));
    assert_eq!(expected, value);
}

#[test]
fn decimal_exp_too_large() {
    let ion_element = read_file_testsuite!("bad/decimalExpTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap().0;
    let expected = crate::IonValue::Decimal(
        BigDecimal::from_str("0.000000000000000000000000000000000000000000025149515645911129")
            .unwrap(),
    );
    assert_eq!(expected, value);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::DecimalExponentTooBig;
    assert_eq!(expected, value);
}

#[test]
fn decimal_len_causes_64_bit_overflow() {
    // let ion_element = read_file_testsuite!("bad/decimalLenCauses64BitOverflow");
    // Not needed due to use of Big Decimal.
}

#[test]
fn decimal_len_too_large() {
    let ion_element = read_file_testsuite!("bad/decimalLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(22));
    assert_eq!(expected, value);
}

#[test]
fn field_name_symbol_id_unmapped() {
    let ion_element = read_file_testsuite!("bad/fieldNameSymbolIDUnmapped");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::SymbolNotFoundInTable;
    assert_eq!(expected, value);
}

#[test]
fn float_len_too_large() {
    let ion_element = read_file_testsuite!("bad/floatLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(7));
    assert_eq!(expected, value);
}

#[test]
fn list_with_value_larger_than_size() {
    let ion_element = read_file_testsuite!("bad/listWithValueLargerThanSize");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::ListLengthWasTooShort;
    assert_eq!(expected, value);
}

#[test]
fn local_symbol_table_with_multiple_imports_fields() {
    let ion_element = read_file_testsuite!("bad/localSymbolTableWithMultipleImportsFields");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn local_symbol_table_with_multiple_symbols_and_imports_fields() {
    let ion_element =
        read_file_testsuite!("bad/localSymbolTableWithMultipleSymbolsAndImportsFields");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn local_symbol_table_with_multiple_symbols_fields() {
    let ion_element = read_file_testsuite!("bad/localSymbolTableWithMultipleSymbolsFields");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn min_long_with_len_too_large() {
    let ion_element = read_file_testsuite!("bad/minLongWithLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(8));
    assert_eq!(expected, value);
}

#[test]
fn min_long_with_len_to_small() {
    let ion_element = read_file_testsuite!("bad/minLongWithLenTooSmall");
    let mut parser = IonParser::new(ion_element);
    let _integer = parser.consume_value().unwrap();
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn negative_int_zero() {
    let ion_element = read_file_testsuite!("bad/negativeIntZero");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidNegativeInt;
    assert_eq!(expected, value);
}

#[test]
fn negative_int_zero_ln() {
    let ion_element = read_file_testsuite!("bad/negativeIntZeroLn");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidNegativeInt;
    assert_eq!(expected, value);
}

#[test]
fn nop_pad_too_short() {
    let ion_element = read_file_testsuite!("bad/nopPadTooShort");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(15));
    assert_eq!(expected, value);
}

#[test]
fn nop_pad_with_annotations() {
    let ion_element = read_file_testsuite!("bad/nopPadWithAnnotations");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn string_len_too_large() {
    let ion_element = read_file_testsuite!("bad/stringLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(38));
    assert_eq!(expected, value);
}

#[test]
fn string_with_latin_encoding() {
    let ion_element = read_file_testsuite!("bad/stringWithLatinEncoding");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NonUtf8String;
    assert_eq!(expected, value);
}

#[test]
fn struct_ordered_empty() {
    let ion_element = read_file_testsuite!("bad/structOrderedEmpty");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::EmptyOrderedStruct;
    assert_eq!(expected, value);
}

#[test]
fn symbol_id_unmapped() {
    let ion_element = read_file_testsuite!("bad/symbolIDUnmapped");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::SymbolNotFoundInTable;
    assert_eq!(expected, value);
}

#[test]
fn symbol_len_too_large() {
    let ion_element = read_file_testsuite!("bad/symbolLenTooLarge");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NotEnoughtDataToRead(1));
    assert_eq!(expected, value);
}

#[test]
fn invalid_version_marker_in_list() {
    let ion_element = read_file_testsuite!("bad/ivmInList");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NestedVersionMarker);
    assert_eq!(expected, value);
}

#[test]
fn invalid_version_marker_in_sexp() {
    let ion_element = read_file_testsuite!("bad/ivmInSexp");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NestedVersionMarker);
    assert_eq!(expected, value);
}

#[test]
fn invalid_version_marker_in_struct() {
    let ion_element = read_file_testsuite!("bad/ivmInStruct");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NestedVersionMarker);
    assert_eq!(expected, value);
}

#[test]
fn invalid_version_marker_in_symbol_table_import() {
    let ion_element = read_file_testsuite!("bad/ivmInSymbolTableImport");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NestedVersionMarker);
    assert_eq!(expected, value);
}

#[test]
fn invalid_version_marker_in_annotation_wrapper() {
    let ion_element = read_file_testsuite!("bad/ivmInAnnotationWrapper");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    // Note, this error should be "NestedVersionMarker", but the binary file 
    // provided by amazon has a wrong length. 
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn struct_ordered_empty_in_list() {
    let ion_element = read_file_testsuite!("bad/structOrderedEmptyInList");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::EmptyOrderedStruct;
    assert_eq!(expected, value);
}
