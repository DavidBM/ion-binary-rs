use crate::ion_data::{from_binary, to_binary, Element, IonData, TimestampPrecision};
use std::fs;
use std::path::Path;

fn text_fixture(relative: &str) -> Vec<u8> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/tests/test-suite")
        .join(relative);
    fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

#[test]
fn upstream_text_symbol_equivalence() {
    let groups = Element::read_all(text_fixture("iontestdata/good/equivs/symbols.ion")).unwrap();
    for group in groups.iter() {
        let values = group.expect_sequence().unwrap();
        let expected = values.iter().last().unwrap();
        // The two unquoted forms are split into multiple values in ion-rs 1.2.0.
        // Upstream fixed this in #1087 after that release.
        let unexpected_split = values.len() > 2
            && matches!(
                expected.as_symbol().and_then(|symbol| symbol.text()),
                Some("$1$2" | "$1a")
            );
        if unexpected_split {
            continue;
        }
        for value in values.iter() {
            assert_eq!(IonData::from(value), IonData::from(expected), "{group}");
        }
    }
}

#[test]
fn upstream_unicode_null_in_a_field_name() {
    let input =
        Element::read_all(text_fixture("iontestdata/good/UnicodeNullInFieldName.ion")).unwrap();
    let binary = to_binary(&input).unwrap();
    let decoded = from_binary(&binary).unwrap();
    let fields = decoded.iter().next().unwrap().expect_struct().unwrap();
    assert_eq!(
        fields.get("f\0o").unwrap().as_symbol().unwrap().text(),
        Some("bar")
    );
}

#[test]
fn native_and_full_model_run_upstream_binary_boolean_conformance() {
    let documents = Element::read_all(text_fixture("conformance/data_model/boolean.ion")).unwrap();
    for document in documents.iter() {
        let parts = document.expect_sequence().unwrap();
        let flattened = parts.iter().last().unwrap().expect_sequence().unwrap();
        let outcome = flattened.iter().last().unwrap().expect_sequence().unwrap();
        let expected = outcome.iter().last().unwrap().as_bool();
        let mut case_count = 0;
        for clause in flattened.iter().filter_map(Element::as_sexp) {
            let mut parts = clause.iter();
            if parts.next().unwrap().as_symbol().unwrap().text() != Some("binary") {
                continue;
            }
            for argument in parts {
                let binary = if let Some(literal) = argument.as_string() {
                    literal
                        .split_whitespace()
                        .map(|byte| u8::from_str_radix(byte, 16).unwrap())
                        .collect()
                } else {
                    let byte: u8 = argument.expect_i64().unwrap().try_into().unwrap();
                    vec![0xE0, 1, 0, 0xEA, byte]
                };
                let native = crate::IonParser::new(binary.as_slice()).consume_all();
                let full_model = from_binary(&binary);
                match expected {
                    Some(boolean) => {
                        assert_eq!(native.unwrap(), [crate::IonValue::Bool(boolean)]);
                        let full_model = full_model.unwrap();
                        assert_eq!(full_model.len(), 1);
                        assert_eq!(full_model.iter().next().unwrap().as_bool(), Some(boolean));
                    }
                    None => assert!(native.is_err() && full_model.is_err()),
                }
                case_count += 1;
            }
        }
        assert!(case_count > 0);
    }
}

#[test]
fn complete_values_survive_binary_roundtrip() {
    let data = Element::read_all(
        "{same: 2024T, same: -0.00, precision: 2024-01-01T00:00:01.012345678901-00:00} $0",
    )
    .unwrap();
    let bytes = to_binary(&data).unwrap();
    let decoded = from_binary(&bytes).unwrap();
    let mut values = decoded.iter();
    let structure = values.next().unwrap().expect_struct().unwrap();
    let mut duplicate_fields = structure.get_all("same");
    let year = duplicate_fields.next().unwrap().expect_timestamp().unwrap();
    assert_eq!(year.precision(), TimestampPrecision::Year);
    assert!(duplicate_fields
        .next()
        .unwrap()
        .expect_decimal()
        .unwrap()
        .coefficient()
        .is_negative_zero());
    assert_eq!(values.next().unwrap().as_symbol().unwrap().text(), None);
    assert_eq!(IonData::from(&decoded), IonData::from(&data));
}
