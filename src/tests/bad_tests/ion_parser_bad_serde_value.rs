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
fn ion_from_bad_serde_1() {}

#[test]
fn ion_from_bad_serde_2() {}
