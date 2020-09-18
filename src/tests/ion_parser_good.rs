use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use crate::read_file_testsuite;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue};

#[test]
fn clob_with_del() {
    let ion_blob = read_file_testsuite!("good/clobWithDel");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(Vec::from([127]))
    );
}
