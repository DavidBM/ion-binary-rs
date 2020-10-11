use crate::{IonEncoder, IonParser, IonValue};
use bigdecimal::BigDecimal;
use chrono::DateTime;
use std::str::FromStr;

#[test]
fn encode_list() {
    let encoder = IonEncoder::new();

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
    let encoder = IonEncoder::new();

    let ion_value = IonValue::List(Vec::new());

    let bytes = encoder.encode_value(&ion_value);

    let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

    assert_eq!(ion_value, resulting_ion_value);
}
