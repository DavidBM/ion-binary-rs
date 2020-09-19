use crate::read_file_testsuite;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue, NullIonValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[test]
fn typecodes_t0() {
    let ion_blob = read_file_testsuite!("good/typecodes/T0");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Null),
    );
}

#[test]
fn typecodes_t1() {
    let ion_blob = read_file_testsuite!("good/typecodes/T1");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Bool(false),);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Bool(true),);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Bool),
    );
}

#[test]
fn typecodes_t2() {
    let ion_blob = read_file_testsuite!("good/typecodes/T2");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Integer(0),);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Integer(255),);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Integer(65535),);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(16777215),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(4294967295),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(1099511627775),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(281474976710655),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(72057594037927935),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"18446744073709551615", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"4722366482869645213695", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"1208925819614629174706175", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"309485009821345068724781055", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"79228162514264337593543950335", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"20282409603651670423947251286015", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(b"5192296858534827628530496329220095", 10).unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Integer),
    );
}

#[test]
fn typecodes_t3() {
    let ion_blob = read_file_testsuite!("good/typecodes/T3");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Integer(-255),);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Integer(-65535),);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(-16777215),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(-4294967295),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(-1099511627775),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(-281474976710655),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Integer(-72057594037927935),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"-18446744073709551615", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"-4722366482869645213695", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"-1208925819614629174706175", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"-309485009821345068724781055", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(BigInt::parse_bytes(b"-79228162514264337593543950335", 10).unwrap()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(b"-20282409603651670423947251286015", 10).unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::BigInteger(
            BigInt::parse_bytes(b"-5192296858534827628530496329220095", 10).unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Integer),
    );
}

#[test]
fn typecodes_t4() {
    let ion_blob = read_file_testsuite!("good/typecodes/T4");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(parser.consume_value().unwrap().0, IonValue::Float32(0.0),);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float32(0.0000000000000000000000000004609175),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Float64(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012497855238365512),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Float),
    );
}

#[test]
fn typecodes_t5() {
    let ion_blob = read_file_testsuite!("good/typecodes/T5");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from(0)),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(BigDecimal::from(0)),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000000000000000000127"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000000000000000032767"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000000000000008388607"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000000000002147483647"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000000000549755813887"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000000140737488355327"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000000036028797018963967"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000000009223372036854775807"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000002361183241434822606847"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000000604462909807314587353087"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000000154742504910672534362390527"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000000039614081257132168796771975167"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Decimal(
            BigDecimal::from_str(
                &"-0.000000000000000000000000000000010141204801825835211973625643007"
            )
            .unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Decimal)
    );
}

#[test]
fn typecodes_t6_large() {
    // This is an strange test as the date is the same but only the coefficient from the
    // decimal part of the seconds changes. But the change is so small that the date
    // remains the same. (Exponent remails -33 for all values)
    let ion_blob = read_file_testsuite!("good/typecodes/T6-large");

    let mut parser = IonParser::new(ion_blob);

    // Seconds coefficient 1000000000000000000000000000000000
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    // Seconds coefficient 1000000000000000000000000000000018
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    // Seconds coefficient 1000000000000000000000000000004626
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    // Seconds coefficient 1000000000000000000000000001184274
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    // Seconds coefficient 1000000000000000000000000303174162
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    // Seconds coefficient 1000000000000000000000077612585490
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    // Seconds coefficient 1000000000000000000019868821885458
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );
}

#[test]
fn typecodes_t6_small() {
    let ion_blob = read_file_testsuite!("good/typecodes/T6-small");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:00:00+00:00").unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:00:00+00:00").unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:00:00+00:00").unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("2401-01-01T00:00:00+00:00").unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:00-00:33").unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::DateTime(
            chrono::DateTime::parse_from_rfc3339("0097-01-01T00:28:01-00:33").unwrap()
        ),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::DateTime),
    );
}

#[test]
fn typecodes_t7_small() {
    let ion_blob = read_file_testsuite!("good/typecodes/T7-small");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Symbol),
    );
}

#[test]
fn typecodes_t7_large() {
	// Another strange tests. This is a bunch of zero symbols with NOP Padding
	// in between.
    let ion_blob = read_file_testsuite!("good/typecodes/T7-large");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("$0".into()),
    );
}

#[test]
fn typecodes_t8() {
    let ion_blob = read_file_testsuite!("good/typecodes/T8");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("0".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("00".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("0000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("00000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("0000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("00000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("000000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("0000000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("00000000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("000000000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("0000000000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::String("00000000000000".into()),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::String),
    );
}

#[test]
fn typecodes_t9() {
    let ion_blob = read_file_testsuite!("good/typecodes/T9");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Clob),
    );
}

#[test]
fn typecodes_t10() {
    let ion_blob = read_file_testsuite!("good/typecodes/T10");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Blob(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Null(NullIonValue::Blob),
    );
}

#[test]
fn typecodes_t11() {
    let ion_blob = read_file_testsuite!("good/typecodes/T11");

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::List(vec![]),
    );

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::List(vec![]),
    );
}
