use crate::{read_file_testsuite, hashmap};
use bigdecimal::BigDecimal;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue, IonParserError, ParsingError, NullIonValue};
use num_bigint::BigInt;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

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
fn int_big_size256() {
    let ion_blob = read_file_testsuite!("good/intBigSize256");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"18173238162219679736857031944447898744767430095109316084451026048678348094928854458274167288816962557611640075817315237016025726423548207924331642028847993938530524659112028449811515920726159569583847554301932799584192974700038250645135419704389244690214111003505621818033044965879076306690914532152840279256440975668846810694285470204245958782248405612488959069641454132691581386219910938587286910894148564397155066367399697230287047229035630842240888106685623631032505806388903066971508775182055551847210338095961815021030725796281642316166745051164958432783938535334657296749823645911331793861360616240344479015948";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
        	BigInt::parse_bytes(expected_number_str, 10).unwrap())
    );
}

#[test]
fn int_big_size14() {
    let ion_blob = read_file_testsuite!("good/intBigSize14");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"2773783639172303802999334644566508";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
        	BigInt::parse_bytes(expected_number_str, 10).unwrap())
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
fn decimal_negative_one_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeOneDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"-1.0").unwrap())
    );
}

#[test]
fn decimal_negative_zero_dot() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeZeroDot");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"-0").unwrap())
    );
}

#[test]
fn decimal_negative_zero_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalNegativeZeroDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"-0.0").unwrap())
    );
}

#[test]
fn decimal_one_dot_zero() {
    let ion_blob = read_file_testsuite!("good/decimalOneDotZero");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"1.0").unwrap())
    );
}

#[test]
fn decimal_zero_dot() {
    let ion_blob = read_file_testsuite!("good/decimalZeroDot");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from_str(&"0.").unwrap())
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
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(0.0)
    );

    // 8000 0000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(-0.0)
    );

    // 4086 6666
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(4.2)
    );

    // c086 6666
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(-4.2)
    );

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
fn int_big_size1201() {
    let ion_blob = read_file_testsuite!("good/intBigSize1201");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"-9c7ff8771c1599f851c220f7ac873291f802a4154dd34ecef5b03dc6b6719488041e37feffd03803637d743b0a0db4dc55d7b09fad276ecb076e6595a07ca98d9b854ddfd527ea8355aa10c2e5c469eb8dc554dfe0f44b2b541cadc8af3da8eeeec8842c4f24f6a9fd18087a60eb42db877e123e8ee680f965f78e075461bc5e00e55eda5bdaf10c05a38e4edb0397320ef21cb065e47187018a97f688eec1ebee37279cfd9433e0f5b00efc635cc346e7fa09bbb6f739ccd1a1a4bf902738bbf0c2ff2519c612806ca75c0d175f4a45d9dab4179425103562882967522dd001cbfd1ee3be16298aca9a8baf9f57ab04e441778c8d7f54e5e919327d427654b73aa1637f4cba2fb2c8301001c5991ca5b601255cca03a1c999f6f44f3a9e25a1d9537ca71986e796253c68369c691abc80ab15b034f85f121f789bf1a04fb0090a17c027a1a1f1179d9feafa852cb727a76c39b3481c71e3bfd7e25600c6c8952d2531382813d56aa254b948f81dd323af6966c68e059770843cb89857181108b8b94967016c4e2be08733f5616cf7ce2bffb3423efb55cccae9b05c80019c0a492a5c45df93b834ed01e8af2495c5425e7f242a3c21b2521071ac9dab65cbd794cfc8dc6ce54bc1a74c7a118d4ea00ea2f57c0dbf25ebf35016fe5bf6158578928271da22b921c5b3529ffc787ead7ca1ebfc498525aed701e67eda360cf7a56f830d510908968217ba23b7eb9aef97f42fb49a5f41b35b075c504f17e44d0e98c921633b2c59592baa428c8554ff9dc3a2b49212872c140799563cd4aafde98a08cc6722b4f28f1b9f1bc8e0497c02a107e8e329b71c54181c28bb30dde847a0877247128846d708febbc9a36d6ec3171cdab0a8ca51d7049755906ce85d8df086c52c8ce0480cdd62f9a52c23257676066797b8449ed4af6ad100dc65fa4b90d3f94b0f49db134e24168328120bcc37f11e51aa37904b0d744c06aad5900c40c512ee0015d8df93283f6fa66997cad482fae4bae8a831f15d6ac59ce334011007c7be88b59c89d6bbd1d11b531c63870d4d13c620c659bff4050db5f32864c7bbd165402a12e70c22f4d64acfbf02f51bac629bcc48efdbb6e8b43f10ba819fad578613cffea7ff6b0519e09f1533c117d60e7fb3368f63448d2ece8ce2871ec81e3e9f9bbf182031aa618e735a46e32797e6ad6a5d1d5f98b537042b62eb05cbde7642d638012d6fb3e2d131bfe99619fd04fa35cc9bb589f54c112b8d81204a947d7e0abea46bafce1358f22a02c78e987b2111921be8c0eb0a9429958d7a74778366196f01e7081098df66ba912f154c2a5ce00eecf481f656e05fa2b925a51223dabc124bc2afb13093b22854528ed75bee2a289be369ea51cf053329f0e1a7a3e231c382e72ab396808c27d6576427005f1effa095319f49a8150e53d9dd028fbb246f1664a1933218f6e15dd0187f6bcf27b9cd4af4f9921bce4ff57843c92ce3f6e07df6af1594b57bd54d7de70315ea19a6cc0ba7c82393bb1e7ff519f719ab5fc183ddedb9b66d4eacfc7d01b6898cbcf0463f476c162b04888a3812359d9012207be7ed3ce854c4bc3e6fd7284f6a35e1f99bb40b793460f3edfe1d80810683d4e05e09c4b0874fd36483d51a4df7ba380d28f8d4815e71f8a909fef2d4b2d79125f855b0cc7b9509626e";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(expected_number_str, 16).unwrap())
    );
}

#[test]
fn int_big_size13() {
    let ion_blob = read_file_testsuite!("good/intBigSize13");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"8f14ca6f603857b79f4a73ccb4";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(expected_number_str, 16).unwrap())
    );
}

#[test]
fn int_big_size16() {
    let ion_blob = read_file_testsuite!("good/intBigSize16");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"fffe15bbbc3ee89af82cabdc68544b2b";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(expected_number_str, 16).unwrap())
    );
}

#[test]
fn int_long_max_value_plus_one() {
    let ion_blob = read_file_testsuite!("good/intLongMaxValuePlusOne");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"8000000000000000";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(expected_number_str, 16).unwrap())
    );
}

#[test]
fn int_long_min_value() {
    let ion_blob = read_file_testsuite!("good/intLongMinValue");

    let mut parser = IonParser::new(ion_blob);

    let expected_number_str = b"-8000000000000000";

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(expected_number_str, 16).unwrap())
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
    // the API returns the Original Symbol the API can not fail. That is defined
    // here: https://amzn.github.io/ion-docs/guides/symbols-guide.html#reading-symboltokens
    // 
    // Additionally, the specification says that the implementation must provide 
    // a way for the user to define their own symbols so that unknown symbols can 
    // be decoded as strings as per the user indications. 
    // 
    // Technically speaking this test shouldn't pass as we return directly the 
    // text for the symbol and not the symbol itself, but we can make it pass
    // (in order to have comprehensive testing) with a user-defined symbol table.
    // 
    // So, after accounting for all that we just add some predefined symbols for 
    // the imported tables in order to make the test pass.

    let ids: Vec<std::string::String> = (1..=10).map(|v| "iopc".to_owned() + &v.to_string()).collect();
    parser.with_shared_table("iopc".to_string(), 1, &ids).unwrap();

    let ids: Vec<std::string::String> = (11..=14277).map(|v| "iopg".to_owned() + &v.to_string()).collect();
    parser.with_shared_table("iopg".to_string(), 1, &ids).unwrap();

    use chrono::DateTime as ChronoDateTime;
    use IonValue::*;

    let expected = IonValue::Annotation((
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
        )))
    ));

    //TODO: Double check that the binary ion really decodes to that structure in another language.


    assert_eq!(
        parser.consume_value().unwrap().0,
        expected
    );
}
