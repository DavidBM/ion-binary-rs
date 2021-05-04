use std::{convert::TryInto, str::FromStr};

use num_bigint::BigInt;
use serde_json::{json, Value};

use crate::{IonValue, NullIonValue};

#[test]
fn serde_from_ion_null() {
    let null_ion = IonValue::Null(NullIonValue::Null);
    let serde_null: Value = null_ion.try_into().unwrap();

    assert_eq!(serde_null, json!(null));
}

#[test]
fn serde_from_ion_bool() {
    let bool_ion = IonValue::Bool(false);
    let serde_bool: Value = bool_ion.try_into().unwrap();

    assert_eq!(serde_bool, json!(false));
}

#[test]
fn serde_from_ion_integer() {
    let integer_ion = IonValue::Integer(18);
    let serde_integer: Value = integer_ion.try_into().unwrap();

    assert_eq!(serde_integer, json!(18));
}

#[test]
fn serde_from_ion_big_integer() {
    let big_int = BigInt::from_str("-201545002204320").unwrap();
    let big_int_ion = IonValue::BigInteger(big_int);
    let serde_big_int: Value = big_int_ion.try_into().unwrap();
}

#[test]
fn serde_from_ion_float() {
    let float_ion = IonValue::Float(36.432);
    let serde_float: Value = float_ion.try_into().unwrap();

    assert_eq!(serde_float, json!(36.432));
}

#[test]
fn serde_from_ion_string() {
    let string_ion = IonValue::String("Argumentum baculinum".to_string());
    let serde_string: Value = string_ion.try_into().unwrap();

    assert_eq!(serde_string, json!("Argumentum baculinum"));
}

#[test]
fn serde_from_ion_list() {}

#[test]
fn serde_from_ion_struct() {}

#[test]
fn ion_from_serde_null() {
    let null_value = json!(null);
    let ion_null: IonValue = null_value.try_into().unwrap();

    assert_eq!(ion_null, IonValue::Null(NullIonValue::Null));
}

#[test]
fn ion_from_serde_bool() {
    let bool_value = json!(true);
    let ion_bool: IonValue = bool_value.try_into().unwrap();

    assert_eq!(ion_bool, IonValue::Bool(true));
}

#[test]
fn ion_from_serde_integer() {
    let integer_value = json!(3);
    let ion_integer: IonValue = integer_value.try_into().unwrap();

    assert_eq!(ion_integer, IonValue::Integer(3));
}

#[test]
fn ion_from_serde_big_integer() {
    let big_int = BigInt::from_str("-201545000003000").unwrap();
}

#[test]
fn ion_from_serde_float() {
    let float_value = json!(23.432);
    let ion_float: IonValue = float_value.try_into().unwrap();

    assert_eq!(ion_float, IonValue::Float(23.432));
}

#[test]
fn ion_from_serde_string() {
    let string_value = json!("Lorem ipsum");
    let ion_string: IonValue = string_value.try_into().unwrap();

    assert_eq!(ion_string, IonValue::String("Lorem ipsum".to_string()));
}

#[test]
fn ion_from_serde_list() {}

#[test]
fn ion_from_serde_struct() {}
