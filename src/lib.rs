//! # Ion Binary parser/encoder & Ion Hash in Rust
//!
//! Parse, encode, and hash Amazon Ion 1.0 binary data in Rust.
//!
//! [`IonParser`], [`IonEncoder`], and [`IonHash`] use the native [`IonValue`]
//! API with string symbols, Rust maps for structs, and Chrono timestamps.
//! Enable `ion-data` for the official `ion-rs` data model and Ion 1.0 text
//! parsing. The feature preserves repeated fields, symbol text availability,
//! signed decimal zero, and timestamp precision in its own value types.
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
//! ## Safety
//!
//! In order to speed up the encoding of data, we use Uninit vector buffers, as otherwise
//! it would require for the buffer to be set to zeroes and then to the correct values.
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

#[cfg(feature = "ion-data")]
pub mod ion_data;

pub use binary_parser_types::ParsingError;
pub use ion_encoder::IonEncoder;
pub use ion_hash::IonHash;
pub use ion_parser::IonParser;
pub use ion_parser_types::{
    IonExtractionError, IonParserError, IonValue, NullIonValue, SerdeJsonParseError,
};
pub use symbol_table::{Symbol, SymbolContextError};
