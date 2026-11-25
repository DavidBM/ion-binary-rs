use crate::IonParser;
use std::fs;
use std::path::{Path, PathBuf};

fn binary_fixtures(root: &Path) -> Vec<PathBuf> {
    let mut fixtures = Vec::new();
    for entry in fs::read_dir(root).unwrap_or_else(|error| panic!("{}: {error}", root.display())) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            fixtures.extend(binary_fixtures(&path));
        } else if path.extension().is_some_and(|extension| extension == "10n") {
            fixtures.push(path);
        }
    }
    fixtures.sort();
    fixtures
}

fn check_binary_fixtures<E: std::fmt::Debug>(
    exceptions: &[&str],
    decode: impl Fn(&Path, &[u8]) -> Result<(), E>,
) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/test-suite/iontestdata");
    let mut mismatches = Vec::new();
    for (section, valid) in [("good", true), ("bad", false)] {
        let section = root.join(section);
        let fixtures = binary_fixtures(&section);
        assert!(!fixtures.is_empty(), "no fixtures in {}", section.display());
        for fixture in fixtures {
            let relative = fixture.strip_prefix(&root).unwrap();
            if exceptions
                .iter()
                .any(|exception| relative == Path::new(exception))
            {
                continue;
            }
            let bytes = fs::read(&fixture).unwrap();
            let result = decode(relative, &bytes);
            if result.is_ok() != valid {
                mismatches.push(format!("{}: {:?}", relative.display(), result));
            }
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn native_binary_fixtures() {
    check_binary_fixtures(&[], |path, bytes| {
        let mut parser = IonParser::new(bytes);
        if path == Path::new("good/item1.10n") {
            super::test_utils::add_item1_symbol_tables(&mut parser);
        }
        parser.consume_all().map(|_| ())
    });
}

#[cfg(feature = "ion-data")]
#[test]
fn full_model_binary_fixtures() {
    check_binary_fixtures(
        &[
            "good/item1.10n", // Stable Element API has no way to provide the imported tables.
            "good/typecodes/T6-large.10n", // ion-rs supports up to 18 fractional digits.
            "good/typecodes/T7-large.10n", // ion-rs rejects wide padded symbol IDs.
        ],
        |_, bytes| crate::ion_data::from_binary(bytes).map(|_| ()),
    );
}
