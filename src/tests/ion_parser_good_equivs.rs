use crate::hashmap;
use crate::ion_parser::IonParser;
use crate::read_file_testsuite;
use crate::IonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[test]
fn equivs_nop_pad_empty_struct() {
    let ion_blob = read_file_testsuite!("good/equivs/nopPadEmptyStruct");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], IonValue::Struct(HashMap::new()));
        assert_eq!(list[1], IonValue::Struct(HashMap::new()));
        assert_eq!(list[2], IonValue::Struct(HashMap::new()));
        assert_eq!(list[0], list[1]);
        assert_eq!(list[1], list[2]);
        assert_eq!(list[0], list[2]);
    } else {
        panic!()
    }
}

#[test]
fn equivs_nop_pad_non_empty_struct() {
    let ion_blob = read_file_testsuite!("good/equivs/nopPadNonEmptyStruct");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
        assert_eq!(list.len(), 3);
        assert_eq!(
            list[0],
            IonValue::Struct(hashmap!("name".into() => IonValue::Bool(true)))
        );
        assert_eq!(
            list[1],
            IonValue::Struct(hashmap!("name".into() => IonValue::Bool(true)))
        );
        assert_eq!(
            list[2],
            IonValue::Struct(hashmap!("name".into() => IonValue::Bool(true)))
        );
        assert_eq!(list[0], list[1]);
        assert_eq!(list[1], list[2]);
        assert_eq!(list[0], list[2]);
    } else {
        panic!()
    }
}

#[test]
fn equivs_padded_ints() {
    let ion_blob = read_file_testsuite!("good/equivs/paddedInts");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], IonValue::Integer(127));
        assert_eq!(list[1], IonValue::Integer(127));
        assert_eq!(list[2], IonValue::Integer(127));
        assert_eq!(list[0], list[1]);
        assert_eq!(list[1], list[2]);
        assert_eq!(list[0], list[2]);
    } else {
        panic!()
    }
}

#[test]
fn equivs_timestamp_fractions() {
    let ion_blob = read_file_testsuite!("good/equivs/timestampFractions");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
        assert_eq!(list.len(), 4);
        assert_eq!(
            list[0],
            IonValue::DateTime(
                chrono::DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00").unwrap()
            )
        );
        assert_eq!(
            list[1],
            IonValue::DateTime(
                chrono::DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00").unwrap()
            )
        );
        assert_eq!(
            list[2],
            IonValue::DateTime(
                chrono::DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00").unwrap()
            )
        );
        assert_eq!(list[0], list[1]);
        assert_eq!(list[1], list[2]);
        assert_eq!(list[0], list[2]);
    } else {
        panic!()
    }
}

#[test]
fn equivs_timestamp_superfluous_offset() {
    let ion_blob = read_file_testsuite!("good/equivs/timestampSuperfluousOffset");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
        assert_eq!(list.len(), 2);
        assert_eq!(
            list[0],
            IonValue::DateTime(
                chrono::DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00").unwrap()
            )
        );
        assert_eq!(
            list[1],
            IonValue::DateTime(
                chrono::DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00").unwrap()
            )
        );
        assert_eq!(list[0], list[1]);
    } else {
        panic!()
    }
}
