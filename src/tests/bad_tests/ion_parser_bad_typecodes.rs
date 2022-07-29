use crate::read_file_testsuite;
use crate::IonParserError;
use crate::ParsingError;
use crate::{binary_parser_types::ValueLength, IonParser};
use std::fs::File;
use std::io::BufReader;

#[test]
fn typecodes_type_14_length_1() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_14_length_1");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::InvalidAnnotationLength(
        ValueLength::ShortLength(1).into(),
    ));
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_14_length_15() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_14_length_15");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NullAnnotationFound;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_14_length_2() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_14_length_2");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::InvalidAnnotationLength(
        ValueLength::ShortLength(2).into(),
    ));
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_0() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_0");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_1() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_1");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_10() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_10");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_11() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_11");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_12() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_12");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_13() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_13");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_14() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_14");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_15() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_15");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_2() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_2");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_3() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_3");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_4() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_4");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_5() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_5");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_6() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_6");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_7() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_7");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_8() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_8");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_15_length_9() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_15_length_9");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidReservedTypeDescriptor;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_10() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_10");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(10).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_11() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_11");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(11).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_12() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_12");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(12).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_13() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_13");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(13).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_14() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_14");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::LongLength.into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_2() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_2");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(2).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_3() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_3");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(3).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_4() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_4");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(4).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_5() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_5");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(5).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_6() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_6");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(6).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_7() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_7");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(7).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_8() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_8");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(8).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_1_length_9() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_1_length_9");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidBoolLength(ValueLength::ShortLength(9).into());
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_3_length_0() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_3_length_0");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidNegativeInt;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_1() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_1");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_10() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_10");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_11() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_11");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_12() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_12");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_13() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_13");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_14() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_14");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_2() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_2");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_3() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_3");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_5() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_5");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_6() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_6");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_7() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_7");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_4_length_9() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_4_length_9");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NotValidLengthFloat;
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_6_length_0() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_6_length_0");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn typecodes_type_6_length_1() {
    let ion_typecode = read_file_testsuite!("bad/typecodes/type_6_length_1");
    let mut parser = IonParser::new(ion_typecode);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}
