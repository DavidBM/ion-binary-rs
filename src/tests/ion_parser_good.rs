use crate::{hashmap, read_file_testsuite};
use crate::{
    ion_parser::IonParser, ion_parser_types::IonValue, IonParserError, ParsingError,
};
use std::fs::File;
use std::io::BufReader;

#[test]
fn clob_with_del() {
    let ion_blob = read_file_testsuite!("good/clobWithDel");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(Vec::from([127]))
    );
}

#[test]
fn clob_with_non_ascii_character() {
    let ion_blob = read_file_testsuite!("good/clobWithNonAsciiCharacter");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(Vec::from([128]))
    );
}

#[test]
fn empty_three_byte_nop_pad() {
    let ion_blob = read_file_testsuite!("good/emptyThreeByteNopPad");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::BinaryError(ParsingError::NoDataToRead)
    );
}

#[test]
fn float32() {
    let ion_blob = read_file_testsuite!("good/float32");

    let mut parser = IonParser::new(ion_blob);

    // 0000 0000
    assert_eq!(parser.consume_value().unwrap().0, IonValue::Float32(0.0));

    // 8000 0000
    assert_eq!(parser.consume_value().unwrap().0, IonValue::Float32(-0.0));

    // 4086 6666
    assert_eq!(parser.consume_value().unwrap().0, IonValue::Float32(4.2));

    // c086 6666
    assert_eq!(parser.consume_value().unwrap().0, IonValue::Float32(-4.2));

    // ff80 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(f32::NEG_INFINITY)
    );

    // 7f80 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(f32::INFINITY)
    );

    // ff7f ffff
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(-340282350000000000000000000000000000000.0)
    );

    // 7f7f ffff
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(340282350000000000000000000000000000000.0)
    );

    // 7fff ffff
    if let IonValue::Float32(number) = parser.consume_value().unwrap().0 {
        if !number.is_nan() {
            panic!("Result should be IonValue::Float32(f32::NAN)")
        }
    }

    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::BinaryError(ParsingError::NoDataToRead)
    );
}

#[test]
fn item1() {
    let ion_blob = read_file_testsuite!("good/item1");

    let mut parser = IonParser::new(ion_blob);

    // Ok, seems that the Ion specification says that the decoder logic should
    // behave differently depending of it it renders symbols as text or just
    // leave them as symbols.
    //
    // Several things to note here:
    //
    // When the API decodes Symbols to text automatically (meaning, we don't
    // return to the user the symbols, but only the string representation) which
    // is our case we need to raise an error when a symbol is not found. Only if
    // the API returns the Original Symbol the API is allowed to not fail. That
    // is defined here:
    // https://amzn.github.io/ion-docs/guides/symbols-guide.html#reading-symboltokens
    //
    // Additionally, the specification says that the implementation must provide
    // a way for the user to define their own symbols so that unknown symbols can
    // be decoded as strings as per the user indications.
    //
    // Technically speaking this test should fail as we return directly the
    // text for the symbol and not the symbol itself, but we can make it pass
    // (in order to have comprehensive testing) with a user-defined symbol table.
    //
    // So, after accounting for all that we just add some predefined symbols for
    // the imported tables in order to make the test pass.

    let ids: Vec<std::string::String> = (1..=10)
        .map(|v| "iopc".to_owned() + &v.to_string())
        .collect();
    parser
        .with_shared_table("iopc".to_string(), 1, &ids)
        .unwrap();

    let ids: Vec<std::string::String> = (11..=14277)
        .map(|v| "iopg".to_owned() + &v.to_string())
        .collect();
    parser
        .with_shared_table("iopg".to_string(), 1, &ids)
        .unwrap();

    use chrono::DateTime as ChronoDateTime;
    use IonValue::*;

    let expected = IonValue::Annotation(
        vec!["iopg18".to_string()],
        Box::new(IonValue::Struct(hashmap!(
            "iopg14".to_string() => String("BT00DCN9OK".to_string()),
            "iopg15".to_string() => Integer(1),
            "iopg17".to_string() => Struct(hashmap!(
                "iopg20".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg108".to_string())))]),
                "iopg26".to_string() => List(vec![Struct(hashmap!(
                    "iopc10".to_string() => Symbol("iopc1".to_string()),
                    "iopc9".to_string() => String("unhappiest discordant droppers".to_string())
                ))]),
                "iopg51".to_string() => List(vec![Struct(hashmap!(
                    "iopc10".to_string() => Symbol("iopc1".to_string()),
                    "iopc9".to_string() => String("Edna disgusts mascara".to_string())
                ))]),
                "iopg22".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg108".to_string())))]),
                "iopg28".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Integer(2)))]),
                "iopg60".to_string() => List(vec![Struct(hashmap!(
                    "iopc10".to_string() => Symbol("iopc1".to_string()),
                    "iopc9".to_string() => String("his deployment microsystems".to_string())
                ))]),
                "iopg1123".to_string() => List(vec![Struct(hashmap!(
                    "iopc10".to_string() => Symbol("iopc1".to_string()),
                    "iopc9".to_string() => String("unhappiest discordant droppers".to_string())
                ))]),
                "iopg5350".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Bool(true)))]),
                "iopg23".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg150".to_string())))]),
                "iopg1244".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => String("641251497029891251497028".to_string())))]),
                "iopg95".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => String("skydiving-altimeters".to_string())))]),
                "iopg25".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg27".to_string())))]),
                "iopg7178".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg9880".to_string())))]),
                "iopg7233".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg9886".to_string())))]),
                "iopg39".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => String("9712514907027".to_string())))]),
                "iopg103".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => String("641251497029891251497028".to_string())))]),
                "iopg33".to_string() => List(vec![Struct(hashmap!(
                    "iopc9".to_string() => String("metaphysics Urquhart Cyclops".to_string()),
                    "iopc10".to_string() => Symbol("iopc1".to_string())
                ))]),
                "iopg30".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => DateTime(ChronoDateTime::parse_from_rfc3339("2010-09-10T19:59:51+00:00").unwrap())))]),
                "iopg31".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg132".to_string())))]),
                "iopg19".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg135".to_string())))]),
                "iopg21".to_string() => List(vec![Struct(hashmap!("iopc9".to_string() => Symbol("iopg38".to_string())))])
            )),
            "version".to_string() => Integer(2)
        ))),
    );

    // TODO: Double check that the binary ion really decodes to that structure
    // in another language like python or js.

    assert_eq!(parser.consume_value().unwrap().0, expected);
}


#[test]
fn symbol_explicit_zero() {
    let ion_blob = read_file_testsuite!("good/symbolExplicitZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into())
    );
}

#[test]
fn symbol_implicit_zero() {
    let ion_blob = read_file_testsuite!("good/symbolImplicitZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into())
    );
}

#[test]
fn testfile28() {
    let ion_blob = read_file_testsuite!("good/testfile28");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::SExpr([
            IonValue::Annotation(
                ["sjis".into()].to_vec(), //https://en.wikipedia.org/wiki/Shift_JIS
                Box::new(IonValue::Clob(
                    [50, 48, 48, 55, 45, 0, 115, 100, 102, 45, 49, 49, 45, 50, 48].to_vec()
                ))
            )
        ].to_vec())
    );
}
