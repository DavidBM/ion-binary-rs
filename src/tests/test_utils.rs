#[macro_export]
macro_rules! read_file_testsuite {
    ($section:expr) => {{
        let mut path = String::from("src/tests/test-suite/iontestdata/");

        path.push_str($section);
        path.push_str(".10n");

        let file =
            File::open(path).unwrap_or_else(|error| panic!("Failed to open file: {:?}", error));

        BufReader::new(file)
    }};
}
