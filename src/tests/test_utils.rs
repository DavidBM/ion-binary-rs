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
