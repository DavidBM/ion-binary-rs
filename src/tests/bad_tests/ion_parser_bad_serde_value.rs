use std::convert::TryInto;

use chrono::DateTime;
use serde_json::Value;

use crate::{IonParserError, IonValue};

#[test]
fn serde_from_ion_symbol() {
    let bad_value = IonValue::Symbol("any".to_string());
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_datetime() {
    let bad_value =
        IonValue::DateTime(DateTime::parse_from_rfc3339("1997-12-11T16:39:27-00:00").unwrap());
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_annotation() {
    let bad_value = IonValue::Annotation(vec!["one".to_string()], Box::new(IonValue::Bool(true)));
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_blob() {
    let bad_value = IonValue::Blob(vec!(2));
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_clob() {
    let bad_value = IonValue::Clob(vec!(3));
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_sexpr() {
    let bad_value = IonValue::SExpr(vec!(IonValue::Bool(true)));
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}
