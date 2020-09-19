use crate::ion_parser::IonParser;
use crate::read_file_testsuite;
use crate::IonValue;
use std::fs::File;
use std::io::BufReader;

#[test]
fn timestamp_timestamp2011() {
    let ion_blob = read_file_testsuite!("good/timestamp/timestamp2011");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    assert_eq!(
        value,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0001-01-01T00:00:00+00:00").unwrap()
        )
    );
}
