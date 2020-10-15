use crate::hashmap;
use crate::{IonEncoder, IonParser, IonValue};
use bigdecimal::BigDecimal;
use chrono::DateTime;
use std::str::FromStr;

#[test]
fn encode_list() {
    let mut encoder = IonEncoder::new();

    let list = vec![
        IonValue::Integer(2523623),
        IonValue::DateTime(DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap()),
        IonValue::String("Hola :D".to_string()),
        IonValue::String("Hello :D".to_string()),
        IonValue::Decimal(BigDecimal::from_str(&"329710294.574576239652439876523876").unwrap()),
        IonValue::String("Test 1".to_string()),
        IonValue::DateTime(DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap()),
        IonValue::String(" ʳ ʴ ʵ ʶ ʷ ʸ ʹ ʺ ʻ ʼ ʽ".to_string()),
        IonValue::String("ഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങച".to_string()),
    ];

    let ion_value = IonValue::List(list);

    let bytes = encoder.encode_value(&ion_value);

    let list = vec![
        IonValue::Integer(2523623),
        IonValue::DateTime(DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap()),
        IonValue::String("Hola :D".to_string()),
        IonValue::String("Hello :D".to_string()),
        IonValue::Blob(bytes),
        IonValue::Decimal(BigDecimal::from_str(&"329710294.574576239652439876523876").unwrap()),
        IonValue::String("Test 1".to_string()),
        IonValue::DateTime(DateTime::parse_from_rfc3339("1996-12-19T16:39:57-00:00").unwrap()),
        IonValue::String(" ʳ ʴ ʵ ʶ ʷ ʸ ʹ ʺ ʻ ʼ ʽ".to_string()),
        IonValue::String("ഇഈഉഊഋഌഎഏഐഒഓഔകഖഗഘങച".to_string()),
    ];

    let ion_value = IonValue::List(list);

    let bytes = encoder.encode_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}

#[test]
fn encode_empty_list() {
    let mut encoder = IonEncoder::new();

    let ion_value = IonValue::List(Vec::new());

    let bytes = encoder.encode_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}

#[test]
fn encode_struct() {
    let mut encoder = IonEncoder::new();

    let expected = hashmap!(
        "Model".to_string() => IonValue::String("CLK 350".to_string()),
        "Type".to_string() => IonValue::String("Sedan".to_string()),
        "Color".to_string() => IonValue::String("White".to_string()),
        "VIN".to_string() => IonValue::String("1C4RJFAG0FC625797".to_string()),
        "Make".to_string() => IonValue::String("Mercedes".to_string()),
        "Year".to_string() => IonValue::Integer(2019)
    );

    let ion_value = IonValue::Struct(expected);

    encoder.add(ion_value.clone());
    let bytes = encoder.encode();

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}
