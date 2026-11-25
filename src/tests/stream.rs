use crate::{IonEncoder, IonParser, IonParserError, IonValue, ParsingError};
use std::collections::HashMap;
use std::io::{self, Read};

const ION_1_0: &[u8] = b"\xE0\x01\x00\xEA";

#[test]
fn version_markers_reset_local_symbols() {
    let mut encoder = IonEncoder::new();
    encoder.add(IonValue::Symbol("local".into()));
    let mut binary = encoder.encode();
    binary.extend_from_slice(ION_1_0);
    binary.extend_from_slice(&[0x71, 10]);

    let mut parser = IonParser::new(&binary[..]);
    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Symbol("local".into())
    );
    assert_eq!(
        parser.consume_value().unwrap_err(),
        IonParserError::SymbolNotFoundInTable
    );
}

#[test]
fn rejects_unsupported_binary_version() {
    let bytes = b"\xE0\x01\x01\xEA\x20";
    assert_eq!(
        IonParser::new(&bytes[..]).consume_all().unwrap_err(),
        IonParserError::BinaryError(ParsingError::UnsupportedVersion(1, 1))
    );
}

#[test]
fn end_of_stream_is_distinct_from_an_incomplete_value() {
    let mut binary = ION_1_0.to_vec();
    binary.push(0x11);
    assert_eq!(
        IonParser::new(&binary[..]).consume_all().unwrap(),
        [IonValue::Bool(true)]
    );
    for truncated in [&[0x81][..], &[0xBE, 0x01], &[0xB1], &[0xE0, 1]] {
        let mut partial = binary.clone();
        partial.extend_from_slice(truncated);
        assert!(IonParser::new(&partial[..]).consume_all().is_err());
    }
}

#[test]
fn handles_repeated_markers_and_padding_iteratively() {
    let mut binary = ION_1_0.repeat(32_768);
    binary.resize(binary.len() + 32_768, 0);
    binary.push(0x11);
    assert_eq!(
        IonParser::new(&binary[..]).consume_all().unwrap(),
        [IonValue::Bool(true)]
    );
}

#[test]
fn symbol_table_annotation_only_applies_to_top_level_structs() {
    let table = "$ion_symbol_table".to_owned();
    let values = vec![
        IonValue::Annotation(vec![table.clone()], Box::new(IonValue::List(vec![]))),
        IonValue::Annotation(
            vec!["note".into(), table.clone()],
            Box::new(IonValue::Struct(HashMap::new())),
        ),
        IonValue::List(vec![IonValue::Annotation(
            vec![table],
            Box::new(IonValue::Struct(HashMap::new())),
        )]),
    ];
    let mut encoder = IonEncoder::new();
    for value in &values {
        encoder.add(value.clone());
    }
    let binary = encoder.encode();
    assert_eq!(IonParser::new(&binary[..]).consume_all().unwrap(), values);
}

#[test]
fn accepts_short_and_interrupted_reads() {
    struct Chunked<'a> {
        data: &'a [u8],
        interrupt: bool,
    }
    impl Read for Chunked<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.interrupt = !self.interrupt;
            if self.interrupt {
                return Err(io::ErrorKind::Interrupted.into());
            }
            let length = buffer.len().min(1);
            self.data.read(&mut buffer[..length])
        }
    }
    let values = [
        IonValue::String("one byte per read".into()),
        IonValue::Blob(vec![9; 512]),
    ];
    let mut encoder = IonEncoder::new();
    for value in &values {
        encoder.add(value.clone());
    }
    let binary = encoder.encode();
    let reader = Chunked {
        data: &binary,
        interrupt: false,
    };
    assert_eq!(IonParser::new(reader).consume_all().unwrap(), values);
}

#[test]
fn a_short_shared_table_keeps_its_declared_symbol_addresses() {
    for version in [1, 2] {
        let import = IonValue::Struct(HashMap::from([
            ("name".into(), IonValue::String("catalog".into())),
            ("version".into(), IonValue::Integer(version)),
            ("max_id".into(), IonValue::Integer(1_000_000_000)),
        ]));
        let table = IonValue::Struct(HashMap::from([
            ("imports".into(), IonValue::List(vec![import])),
            (
                "symbols".into(),
                IonValue::List(vec![IonValue::String("local".into())]),
            ),
        ]));
        let mut encoder = IonEncoder::new();
        encoder.add(IonValue::Annotation(
            vec!["$ion_symbol_table".into()],
            Box::new(table),
        ));
        let mut binary = encoder.encode();
        binary.extend_from_slice(&[0x71, 10, 0x74]);
        binary.extend_from_slice(&1_000_000_010_u32.to_be_bytes());

        let mut parser = IonParser::new(&binary[..]);
        parser
            .with_shared_table("catalog".into(), 1, &["imported".into()])
            .unwrap();
        assert_eq!(
            parser.consume_all().unwrap(),
            [
                IonValue::Symbol("imported".into()),
                IonValue::Symbol("local".into())
            ]
        );
    }
}

#[test]
fn shared_table_versions_preserve_the_older_symbol_ids() {
    let mut parser = IonParser::new(&[][..]);
    parser
        .with_shared_table("catalog".into(), 2, &["first".into(), "second".into()])
        .unwrap();
    assert_eq!(
        parser.with_shared_table("catalog".into(), 1, &["invalid".into()]),
        Err(crate::SymbolContextError::NewTableIsNotSuperSetOfPrevious),
    );
    parser
        .with_shared_table("catalog".into(), 1, &["first".into()])
        .unwrap();
}
