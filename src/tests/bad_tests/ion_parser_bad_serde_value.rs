use crate::IonValue;

#[test]
fn serde_from_ion_symbol() {
    let value = IonValue::Symbol("any".to_string());
}

#[test]
fn serde_from_ion_datetime() {}

#[test]
fn ion_from_bad_serde_1() {}

#[test]
fn ion_from_bad_serde_2() {}
