use ion_binary_rs::{IonEncoder, IonParser, IonParserError, IonValue, ParsingError};

fn encode_nested_list(depth: usize) -> Vec<u8> {
    let value = (0..depth).fold(IonValue::Integer(1), |value, _| IonValue::List(vec![value]));
    let mut encoder = IonEncoder::new();
    encoder.add(value);
    encoder.encode()
}

#[test]
fn native_parser_checks_the_nesting_limit() {
    let boundary = encode_nested_list(128);
    assert!(IonParser::new(boundary.as_slice()).consume_all().is_ok());

    let too_deep = encode_nested_list(129);
    assert!(matches!(
        IonParser::new(too_deep.as_slice()).consume_value(),
        Err(IonParserError::BinaryError(
            ParsingError::NestingLimitExceeded(128)
        ))
    ));
}
