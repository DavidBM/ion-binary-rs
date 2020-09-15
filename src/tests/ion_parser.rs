use std::collections::HashMap;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue};
use bytes::buf::BufExt;
use env_logger::Env;

#[test]
fn decode_full_ion() {
    let _ = env_logger::from_env(Env::default().default_filter_or("trace")).is_test(true).try_init();

    let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";

    let mut parser = IonParser::new(ion_test.reader());

    let mut expected = HashMap::new();

    expected.insert("Model".to_string(), IonValue::String("CLK 350".to_string()));
    expected.insert("Type".to_string(), IonValue::String("Sedan".to_string()));
    expected.insert("Color".to_string(), IonValue::String("White".to_string()));
    expected.insert("VIN".to_string(), IonValue::String("1C4RJFAG0FC625797".to_string()));
    expected.insert("Make".to_string(), IonValue::String("Mercedes".to_string()));
    expected.insert("Year".to_string(), IonValue::Integer(2019));

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Struct(expected));
}
