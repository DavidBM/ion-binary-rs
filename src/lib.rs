//! # Ion Binary in Rust
//!
//! Ion binary is a library written in safe rust for parsing Amazon's Ion binary format.
//!
//! [![Coverage Status](https://coveralls.io/repos/github/Couragium/ion-binary-rs/badge.svg?branch=master)](https://coveralls.io/github/Couragium/ion-binary-rs?branch=master) 
//! [![Buils Status](https://github.com/Couragium/ion-binary-rs/workflows/Rust/badge.svg)](https://github.com/Couragium/ion-binary-rs/actions) 
//! [![Documentation](https://docs.rs/ion-binary-rs/badge.svg)](https://docs.rs/ion-binary-rs) 
//! [![Crates.io](https://img.shields.io/crates/v/ion-binary-rs)](https://crates.io/crates/ion-binary-rs) 
//!
//! It doesn't handle the text format and it cannot encode complex structures, just the
//! primitives but it can parse any Ion blob you find.
//!
//! ## How to use the library
//!
//! First of all, you need to be aware of the trade offs that we took for this library:
//!
//! - The API returns strings instead of Symbols. If needed we can add symbol, but we
//! think string is the simpler and safer bet for now.
//! - You can add shared tables for binary blobs that doesn't have all the required
//! symbols
//!
//! We have implemented (and still are) the amazon ion test-suite. So you can check all the examples.
//!
//! ## Example
//!
//! TODO
//!

pub(crate) mod binary_parser;
pub(crate) mod binary_parser_types;
pub(crate) mod ion_parser;
pub(crate) mod ion_parser_types;
pub(crate) mod symbol_table;

#[cfg(test)]
mod tests;

pub use binary_parser_types::ParsingError;
pub use ion_parser::IonParser;
pub use ion_parser_types::{IonParserError, IonValue, NullIonValue};
pub use symbol_table::{Symbol, SymbolContextError};
