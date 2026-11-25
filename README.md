<!-- cargo-sync-readme start -->

# Ion Binary parser/encoder & Ion Hash in Rust

Parse, encode, and hash Amazon Ion 1.0 binary data in Rust.

[![Documentation](https://docs.rs/ion-binary-rs/badge.svg)](https://docs.rs/ion-binary-rs)
[![Crates.io](https://img.shields.io/crates/v/ion-binary-rs)](https://crates.io/crates/ion-binary-rs)

`IonParser`, `IonEncoder`, and `IonHash` use the native `IonValue` API. `IonValue`
uses string symbols, Rust maps for structs, and Chrono timestamps. Configure
shared symbol imports with `IonParser::with_shared_table`.

The native parser rejects values nested more than 128 levels deep. Bound input
size and choose `consume_value` to process streams without keeping all decoded
top-level values in memory. The optional `ion_data` APIs use the upstream reader
and its own limits.

## Ion data model

Enable the `ion-data` Cargo feature to use the official `ion-rs` value model.
It supports Ion 1.0 text and binary, repeated field names, unknown symbol text,
signed decimal zero, timestamp precision, and Ion equivalence. These values
stay in the `ion_data` API so those distinctions survive binary round trips.

```rust
use ion_binary_rs::ion_data::{from_binary, to_binary, Element, IonData, IonResult};

fn main() -> IonResult<()> {
    let values = Element::read_all("{item: 2024T, item: -0.00}")?;
    let bytes = to_binary(&values)?;
    assert_eq!(IonData::from(from_binary(&bytes)?), IonData::from(values));
    Ok(())
}
```

The `ion_data` binary reader supports up to 18 fractional timestamp digits.
It uses the stable upstream reader API, which provides no catalog injection.
The released `ion-rs` 1.2.0 text parser splits unquoted `$1$2` and `$1a` into
multiple tokens. Single-quote them to preserve the symbol text. The correction
is on main. [Upstream fix](https://github.com/amazon-ion/ion-rust/pull/1087)
The crate supports Ion 1.0. Ion 1.1 version markers return an error.

## Example

### Parsing

```rust,no_run

use ion_binary_rs::IonParser;

// This is the response from Amazon's QLDB introduction example using Rusoto
let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";

let mut parser = IonParser::new(&ion_test[..]);

println!("Decoded Ion: {:?}", parser.consume_all().unwrap())
// Decoded Ion: [Struct({"Color": String("White"), "Year": Integer(2019), "VIN": String("1C4RJFAG0FC625797"), "Make": String("Mercedes"), "Model": String("CLK 350"), "Type": String("Sedan")})]

```

### Encoding

```rust,no_run

use ion_binary_rs::{IonEncoder, IonParser, IonValue};
use std::collections::HashMap;

let mut ion_struct = HashMap::new();

ion_struct.insert("Model".to_string(), IonValue::String("CLK 350".to_string()));
ion_struct.insert("Type".to_string(), IonValue::String("Sedan".to_string()));
ion_struct.insert("Color".to_string(), IonValue::String("White".to_string()));
ion_struct.insert(
    "VIN".to_string(),
    IonValue::String("1C4RJFAG0FC625797".to_string()),
);
ion_struct.insert("Make".to_string(), IonValue::String("Mercedes".to_string()));
ion_struct.insert("Year".to_string(), IonValue::Integer(2019));

let ion_value = IonValue::Struct(ion_struct);

let mut encoder = IonEncoder::new();

encoder.add(ion_value.clone());
let bytes = encoder.encode();

let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

assert_eq!(ion_value, resulting_ion_value);
```

### Hashing

```rust,no_run
use sha2::Sha256;
use ion_binary_rs::{IonHash, IonValue};
use std::collections::HashMap;

let mut ion_struct = HashMap::new();

ion_struct.insert("Model".to_string(), IonValue::String("CLK 350".to_string()));
ion_struct.insert("Type".to_string(), IonValue::String("Sedan".to_string()));
ion_struct.insert("Color".to_string(), IonValue::String("White".to_string()));
ion_struct.insert(
    "VIN".to_string(),
    IonValue::String("1C4RJFAG0FC625797".to_string()),
);
ion_struct.insert("Make".to_string(), IonValue::String("Mercedes".to_string()));
ion_struct.insert("Year".to_string(), IonValue::Integer(2019));

let ion_value = IonValue::Struct(ion_struct);

let hash = IonHash::digest::<Sha256>(&ion_value);

println!("{:X?}", hash);
```

## Safe Rust

No unsafe code was directly used in this crate. You can check in lib.rs
the `#![deny(unsafe_code)]` line.

## Contributing

Run `git submodule update --init` and `cargo test --all-features` from a Git
checkout. The test suite discovers `.10n` files in the official binary fixture
corpus, with value assertions in the dedicated tests. The `ion-data` feature
also runs official text symbol-equivalence fixtures, the embedded-NUL field
fixture, and binary boolean cases from the upstream conformance DSL. The
optional reader's three upstream fixture exceptions are recorded in
`src/tests/upstream_binary.rs`.

We would be thrilled if you decide to check the library and/or contribute to it!
Just open an issue or pull request and we can check what you would like to implement.
Bug hunting and proposals are always welcomed. And of course, feel free to ask anything.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>


<!-- cargo-sync-readme end -->
