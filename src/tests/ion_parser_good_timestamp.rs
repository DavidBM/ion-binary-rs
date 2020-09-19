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
            chrono::DateTime::parse_from_rfc3339("2011-01-01T00:00:00+00:00").unwrap()
        )
    );
}

#[test]
fn timestamp_timestamp2011_02() {
    let ion_blob = read_file_testsuite!("good/timestamp/timestamp2011-02");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    assert_eq!(
        value,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-01T00:00:00+00:00").unwrap()
        )
    );
}

#[test]
fn timestamp_timestamp2011_02_20() {
    let ion_blob = read_file_testsuite!("good/timestamp/timestamp2011-02-20");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    assert_eq!(
        value,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2011-02-20T00:00:00+00:00").unwrap()
        )
    );
}

#[test]
fn timestamp_timestamp2011_02_20_t19_30_59_100_08_00() {
	//2011-02-20T11_30_59_100-08_00 -> Explanation for the different hour in the filename 
	//and the content in the comment bellow.
    let ion_blob = read_file_testsuite!("good/timestamp/timestamp2011-02-20T19_30_59_100-08_00");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    assert_eq!(
        value,
        IonValue::DateTime(
        	// Note: In the binary the values are in UTC, but in the filename, the date has 
        	// the same values as UTC but with a timezone, which is not correct. The binary 
        	// content and the filename are two different dates. (I hope I'm right). That 
        	// is why we changes from the hour 19 (file name) to the hour 11 in timezone -8
        	// as the binary contains a 19.
            chrono::DateTime::parse_from_rfc3339("2011-02-20T11:30:59.100-08:00").unwrap()
        )
    );
}
