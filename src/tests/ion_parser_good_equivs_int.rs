use crate::read_file_testsuite;
use crate::ion_parser::IonParser;
use std::fs::File;
use std::io::BufReader;
use crate::IonValue;

#[test]
fn equivs_ints_large_negative1() {
    let ion_blob = read_file_testsuite!("good/equivs/intsLargeNegative1");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
    	assert_eq!(list[0], list[1]);
    } else {
    	panic!()
    }
}

#[test]
fn equivs_ints_large_negative2() {
    let ion_blob = read_file_testsuite!("good/equivs/intsLargeNegative2");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
    	assert_eq!(list[0], list[1]);
    } else {
    	panic!()
    }
}

#[test]
fn equivs_ints_large_negative3() {
    let ion_blob = read_file_testsuite!("good/equivs/intsLargeNegative3");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
    	assert_eq!(list[0], list[1]);
    } else {
    	panic!()
    }
}

#[test]
fn equivs_ints_large_positive1() {
    let ion_blob = read_file_testsuite!("good/equivs/intsLargePositive1");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
    	assert_eq!(list[0], list[1]);
    } else {
    	panic!()
    }
}

#[test]
fn equivs_ints_large_positive2() {
    let ion_blob = read_file_testsuite!("good/equivs/intsLargePositive2");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
    	assert_eq!(list[0], list[1]);
    } else {
    	panic!()
    }
}

#[test]
fn equivs_ints_large_positive3() {
    let ion_blob = read_file_testsuite!("good/equivs/intsLargePositive3");

    let mut parser = IonParser::new(ion_blob);

    let value = parser.consume_value().unwrap().0;

    if let IonValue::SExpr(ref list) = value {
    	assert_eq!(list[0], list[1]);
    } else {
    	panic!()
    }
}
