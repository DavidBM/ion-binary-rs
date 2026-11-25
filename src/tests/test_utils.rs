#[macro_export]
macro_rules! read_file_testsuite {
    ($section:expr) => {{
        let mut path = std::string::String::from("src/tests/test-suite/iontestdata/");

        path.push_str($section);
        path.push_str(".10n");

        let file =
            File::open(path).unwrap_or_else(|error| panic!("Failed to open file: {:?}", error));

        BufReader::new(file)
    }};
}

#[macro_export]
macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub(super) fn add_item1_symbol_tables(parser: &mut crate::IonParser<impl std::io::Read>) {
    for (name, range) in [("iopc", 1..=10), ("iopg", 11..=14_277)] {
        let symbols = range
            .map(|index| format!("{name}{index}"))
            .collect::<Vec<_>>();
        parser.with_shared_table(name.into(), 1, &symbols).unwrap();
    }
}
