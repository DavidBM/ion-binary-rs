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
    let bad_value = IonValue::Blob(vec![2]);
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_clob() {
    let bad_value = IonValue::Clob(vec![3]);
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_sexpr() {
    let bad_value = IonValue::SExpr(vec![IonValue::Bool(true)]);
    let result: Result<Value, IonParserError> = bad_value.clone().try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::TypeNotSupported(bad_value));
}

#[test]
fn serde_from_ion_nan() {
    let bad_value = IonValue::Float(f64::NAN);
    let result: Result<Value, IonParserError> = bad_value.try_into();
    let error = result.unwrap_err();
    match error {
        IonParserError::DecimalNotANumericValue(x) => {
            assert_eq!(true, x.is_nan());
        }
        _ => {
            assert_eq!(true, false);
        }
    }
}

#[test]
fn serde_from_ion_infinity() {
    let infinity = f64::INFINITY;
    let bad_value = IonValue::Float(infinity);
    let result: Result<Value, IonParserError> = bad_value.try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::DecimalNotANumericValue(infinity));
}

#[test]
fn serde_from_ion_neg_infinity() {
    let neg_infinity = f64::NEG_INFINITY;
    let bad_value = IonValue::Float(neg_infinity);
    let result: Result<Value, IonParserError> = bad_value.try_into();
    let error = result.unwrap_err();

    assert_eq!(error, IonParserError::DecimalNotANumericValue(neg_infinity));
}
