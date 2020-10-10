use crate::ion_parser::IonParser;
use crate::read_file_testsuite;
use crate::IonParserError;
use crate::ParsingError;
use std::fs::File;
use std::io::BufReader;

#[test]
fn annotation_container_too_long() {
    let ion_annotation = read_file_testsuite!("bad/annotationLengthTooLongContainer");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BadAnnotationLength;
    assert_eq!(expected, value);
}

#[test]
fn annotation_scalar_too_long() {
    let ion_annotation = read_file_testsuite!("bad/annotationLengthTooLongScalar");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BadAnnotationLength;
    assert_eq!(expected, value);
}

#[test]
fn annotation_container_too_short() {
    let ion_annotation = read_file_testsuite!("bad/annotationLengthTooShortContainer");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BadAnnotationLength;
    assert_eq!(expected, value);
}

#[test]
fn annotation_scalar_too_short() {
    let ion_annotation = read_file_testsuite!("bad/annotationLengthTooShortScalar");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BadAnnotationLength;
    assert_eq!(expected, value);
}

#[test]
fn annotation_nested() {
    let ion_annotation = read_file_testsuite!("bad/annotationNested");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NestedAnnotations;
    assert_eq!(expected, value);
}

#[test]
fn annotation_symbol_id_unmapped() {
    let ion_annotation = read_file_testsuite!("bad/annotationSymbolIDUnmapped");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::SymbolIdNotDefined;
    assert_eq!(expected, value);
}

#[test]
fn annotation_with_no_value() {
    let ion_annotation = read_file_testsuite!("bad/annotationWithNoValue");
    let mut parser = IonParser::new(ion_annotation);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::BinaryError(ParsingError::NoDataToRead);
    assert_eq!(expected, value);
}

#[test]
fn empty_annotated_int() {
    let ion_element = read_file_testsuite!("bad/emptyAnnotatedInt");
    let mut parser = IonParser::new(ion_element);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::NullAnnotationFound;
    assert_eq!(expected, value);
}
