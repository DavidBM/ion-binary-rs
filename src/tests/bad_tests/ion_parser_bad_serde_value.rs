use crate::IonValue;

#[test]
fn serde_from_ion_symbol() {
    let value = IonValue::Symbol("any".to_string());
}

#[test]
fn serde_from_ion_list() {}

#[test]
fn ion_from_serde_symbol() {}

#[test]
fn ion_from_serde_list() {}
