use crate::ion_parser::IonParser;
use crate::read_file_testsuite;
use crate::IonParserError;
use std::fs::File;
use std::io::BufReader;

#[test]
fn timestamp_leap_day_non_leap_year_1() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/outOfRange/leapDayNonLeapYear_1");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidDate(2001, 2, 29, 0, 0, 0, 0);
    assert_eq!(expected, value);
}

#[test]
fn timestamp_leap_day_non_leap_year_2() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/outOfRange/leapDayNonLeapYear_2");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidDate(2001, 2, 29, 0, 0, 0, 0);
    assert_eq!(expected, value)
}

#[test]
fn timestamp_fraction_10d_1() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampFraction10d-1");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::Unimplemented;
    assert_eq!(expected, value);
}

#[test]
fn timestamp_fraction_11d_1() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampFraction11d-1");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::Unimplemented;
    assert_eq!(expected, value);
}

#[test]
fn timestamp_fraction_1d_0() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampFraction1d0");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::Unimplemented;
    assert_eq!(expected, value);
}

#[test]
fn timestamp_hour_without_minute() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampHourWithoutMinute");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::Unimplemented;
    assert_eq!(expected, value);
}

#[test]
fn timestamp_len_too_large() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampLenTooLarge");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::DateValueTooBig;
    assert_eq!(expected, value);
}

#[test]
fn timestamp_negative_fraction() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampNegativeFraction");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::Unimplemented;
    assert_eq!(expected, value);
}

#[test]
fn timestamp_sept_31() {
    let ion_timestamp = read_file_testsuite!("bad/timestamp/timestampSept31");
    let mut parser = IonParser::new(ion_timestamp);
    let value = parser.consume_value().unwrap_err();
    let expected = IonParserError::InvalidDate(2015, 9, 31, 0, 0, 0, 0);
    assert_eq!(expected, value);
}
