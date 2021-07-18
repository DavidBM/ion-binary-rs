//! # Ion Binary parser/encoder & Ion Hash in Rust
//!
//! Ion binary is a production-ready library written in safe rust for parsing, encoding and hashing Amazon's Ion binary format.
//!
//! [![Coverage Status](https://coveralls.io/repos/github/Couragium/ion-binary-rs/badge.svg?branch=master)](https://coveralls.io/github/Couragium/ion-binary-rs?branch=master)
//! [![Buils Status](https://github.com/Couragium/ion-binary-rs/workflows/Rust/badge.svg)](https://github.com/Couragium/ion-binary-rs/actions)
//! [![Documentation](https://docs.rs/ion-binary-rs/badge.svg)](https://docs.rs/ion-binary-rs)
//! [![Crates.io](https://img.shields.io/crates/v/ion-binary-rs)](https://crates.io/crates/ion-binary-rs)
//!
//! It should be able to **parse**, **encode** and **hash** anything you throw at it. Any failure to do so
//! is a bug that we will fix and we will be very happy if you report them.
//!
//! ## How to use the library
//!
//! First of all, you need to be aware of the trade offs that we took for this library:
//!
//! - The API returns strings instead of Symbols. If needed we can add symbol, but we
//! think string is the the most ergonomic way.
//! - When parsing/decoding you can add shared tables for binary blobs that doesn't have
//! all the required symbols.
//!
//! We have implemented the whole amazon ion test-suite for parsing.
//! Encoding and Hashing fully tested. We are working in expading the coverage.
//! We would appreciate any bug you can report. You can check all the test for examples.
//!
//! ## Example
//!
//! ### Parsing
//!
//! ```rust,no_run
//!
//! use ion_binary_rs::IonParser;
//!
//! // This is the response from Amazon's QLDB introduction example using Rusoto
//! let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";
//!
//! let mut parser = IonParser::new(&ion_test[..]);
//!
//! println!("Decoded Ion: {:?}", parser.consume_all().unwrap())
//! // Decoded Ion: [Struct({"Color": String("White"), "Year": Integer(2019), "VIN": String("1C4RJFAG0FC625797"), "Make": String("Mercedes"), "Model": String("CLK 350"), "Type": String("Sedan")})]
//!
//! ```
//!
//! ### Encoding
//!
//! ```rust,no_run
//!
//! use ion_binary_rs::{IonEncoder, IonParser, IonValue};
//! use std::collections::HashMap;
//!
//! let mut ion_struct = HashMap::new();
//!
//! ion_struct.insert("Model".to_string(), IonValue::String("CLK 350".to_string()));
//! ion_struct.insert("Type".to_string(), IonValue::String("Sedan".to_string()));
//! ion_struct.insert("Color".to_string(), IonValue::String("White".to_string()));
//! ion_struct.insert(
//!     "VIN".to_string(),
//!     IonValue::String("1C4RJFAG0FC625797".to_string()),
//! );
//! ion_struct.insert("Make".to_string(), IonValue::String("Mercedes".to_string()));
//! ion_struct.insert("Year".to_string(), IonValue::Integer(2019));
//!
//! let ion_value = IonValue::Struct(ion_struct);
//!
//! let mut encoder = IonEncoder::new();
//!
//! encoder.add(ion_value.clone());
//! let bytes = encoder.encode();
//!
//! let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;
//!
//! assert_eq!(ion_value, resulting_ion_value);
//! ```
//!
//! ### Hashing
//!
//! ```rust,no_run
//! use sha2::Sha256;
//! use ion_binary_rs::{IonHash, IonValue};
//! use std::collections::HashMap;
//!
//! let mut ion_struct = HashMap::new();
//!
//! ion_struct.insert("Model".to_string(), IonValue::String("CLK 350".to_string()));
//! ion_struct.insert("Type".to_string(), IonValue::String("Sedan".to_string()));
//! ion_struct.insert("Color".to_string(), IonValue::String("White".to_string()));
//! ion_struct.insert(
//!     "VIN".to_string(),
//!     IonValue::String("1C4RJFAG0FC625797".to_string()),
//! );
//! ion_struct.insert("Make".to_string(), IonValue::String("Mercedes".to_string()));
//! ion_struct.insert("Year".to_string(), IonValue::Integer(2019));
//!
//! let ion_value = IonValue::Struct(ion_struct);
//!
//! let hash = IonHash::digest::<Sha256>(&ion_value);
//!
//! println!("{:X?}", hash);
//! ```
//!
//! ## Safe Rust
//!
//! No unsafe code was directly used in this crate. You can check in lib.rs
//! the `#![deny(unsafe_code)]` line.
//!
//! ## Contributing
//!
//! We would be thrilled if you decide to check the library and/or contribute to it!
//! Just open an issue or pull request and we can check what you would like to implement.
//! Bug hunting and proposals are always welcomed. And of course, feel free to ask anything.
//!
//! ## License
//!
//! <sup>
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//! </sup>
//!
//! <br/>
//!
//! <sub>
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.
//! </sub>
//!

#![deny(unsafe_code)]

pub(crate) mod binary_encoder;
pub(crate) mod binary_parser;
pub(crate) mod binary_parser_types;
pub(crate) mod ion_encoder;
pub(crate) mod ion_hash;
pub(crate) mod ion_hash_encoder;
pub(crate) mod ion_parser;
pub(crate) mod ion_parser_types;
pub(crate) mod ion_value_impl;
pub(crate) mod symbol_table;

#[cfg(test)]
mod tests;

pub use binary_parser_types::ParsingError;
pub use ion_encoder::IonEncoder;
pub use ion_hash::IonHash;
pub use ion_parser::IonParser;
pub use ion_parser_types::{
    IonExtractionError, IonParserError, IonValue, NullIonValue, SerdeJsonParseError,
};
pub use symbol_table::{Symbol, SymbolContextError};
