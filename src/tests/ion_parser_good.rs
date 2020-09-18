use crate::tests::test_utils::read_file;
use std::path::Path;
use crate::{ion_parser::IonParser, ion_parser_types::IonValue};

#[test]
fn clob_with_del() {
    let ion_blob = read_file(Path::new("src/tests/test-suite/iontestdata/good/clobWithDel.10n"));

    let mut parser = IonParser::new(ion_blob);

    assert_eq!(
        parser.consume_value().unwrap().0,
        IonValue::Clob(Vec::from([127]))
    );
}
