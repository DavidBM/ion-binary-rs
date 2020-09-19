use crate::read_file_testsuite;
use crate::{
    ion_parser::IonParser, ion_parser_types::IonValue, NullIonValue,
};
use std::fs::File;
use std::io::BufReader;

#[test]
fn null() {
    let ion_blob = read_file_testsuite!("good/null");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Null)
    );
}

#[test]
fn null_blob() {
    let ion_blob = read_file_testsuite!("good/nullBlob");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Blob)
    );
}

#[test]
fn null_bool() {
    let ion_blob = read_file_testsuite!("good/nullBool");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Bool)
    );
}

#[test]
fn null_clob() {
    let ion_blob = read_file_testsuite!("good/nullClob");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Clob)
    );
}

#[test]
fn null_decimal() {
    let ion_blob = read_file_testsuite!("good/nullDecimal");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Decimal)
    );
}

#[test]
fn null_float() {
    let ion_blob = read_file_testsuite!("good/nullFloat");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Float)
    );
}

#[test]
fn null_int2() {
    let ion_blob = read_file_testsuite!("good/nullInt2");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Integer)
    );
}

#[test]
fn null_int3() {
    let ion_blob = read_file_testsuite!("good/nullInt3");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Integer)
    );
}

#[test]
fn null_list() {
    let ion_blob = read_file_testsuite!("good/nullList");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::List)
    );
}

#[test]
fn null_sexp() {
    let ion_blob = read_file_testsuite!("good/nullSexp");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::SExpr)
    );
}

#[test]
fn null_string() {
    let ion_blob = read_file_testsuite!("good/nullString");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::String)
    );
}

#[test]
fn null_struct() {
    let ion_blob = read_file_testsuite!("good/nullStruct");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Struct)
    );
}

#[test]
fn null_symbol() {
    let ion_blob = read_file_testsuite!("good/nullSymbol");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Symbol)
    );
}

#[test]
fn null_timestamp() {
    let ion_blob = read_file_testsuite!("good/nullTimestamp");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::DateTime)
    );
}
