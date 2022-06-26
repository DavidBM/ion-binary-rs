use crate::{hashmap, read_file_testsuite};
use crate::{ion_parser::IonParser, ion_parser_types::IonValue, NullIonValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[test]
fn struct_annotated_empty() {
    let ion_blob = read_file_testsuite!("good/structAnnotatedEmpty");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Annotation(
            ["max_id".to_string()].to_vec(),
            Box::new(IonValue::Struct(HashMap::new()))
        )
    );
}

#[test]
fn struct_annotated_ordered() {
    let ion_blob = read_file_testsuite!("good/structAnnotatedOrdered");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Annotation(
            ["symbols".to_string(), "max_id".to_string()].to_vec(),
            Box::new(IonValue::Struct(hashmap!(
                "version".to_string() => IonValue::Bool(false),
                "imports".to_string() => IonValue::Bool(true),
                "name".to_string() => IonValue::Null(NullIonValue::Null)
            )))
        )
    );
}

#[test]
fn struct_empty() {
    let ion_blob = read_file_testsuite!("good/structEmpty");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(HashMap::new())
    );
}

#[test]
fn struct_len13() {
    let ion_blob = read_file_testsuite!("good/structLen13");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!(
            "name".into() => IonValue::String("123456789AB".into())
        ))
    );
}

#[test]
fn struct_len14() {
    let ion_blob = read_file_testsuite!("good/structLen14");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!(
            "name".into() => IonValue::String("123456789ABC".into())
        ))
    );
}

#[test]
fn struct_len15() {
    let ion_blob = read_file_testsuite!("good/structLen15");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!(
            "name".into() => IonValue::String("123456789ABCD".into())
        ))
    );
}

#[test]
fn struct_ordered() {
    let ion_blob = read_file_testsuite!("good/structOrdered");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!(
            "version".to_string() => IonValue::Bool(false),
            "imports".to_string() => IonValue::Bool(true),
            "name".to_string() => IonValue::Null(NullIonValue::Null)
        ))
    );
}

#[test]
fn struct_ordered_in_list() {
    let ion_blob = read_file_testsuite!("good/structOrderedInList");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::List(vec![
            IonValue::Struct(hashmap!(
                "version".to_string() => IonValue::Bool(false),
                "imports".to_string() => IonValue::Bool(true),
                "name".to_string() => IonValue::Null(NullIonValue::Null)
            ))
        ])
    );
}

#[test]
fn struct_unordered() {
    let ion_blob = read_file_testsuite!("good/structUnordered");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Struct(hashmap!(
            "version".to_string() => IonValue::Bool(false),
            "imports".to_string() => IonValue::Bool(true),
            "name".to_string() => IonValue::Null(NullIonValue::Null)
        ))
    );
}
