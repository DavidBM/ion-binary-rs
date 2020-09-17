pub(crate) mod binary_parser;
pub(crate) mod binary_parser_types;
pub(crate) mod ion_parser;
pub(crate) mod ion_parser_types;
pub(crate) mod symbol_table;

#[cfg(test)]
mod tests;

pub use ion_parser::IonParser;
pub use ion_parser_types::{IonParserError, IonValue};

// TODO: Add tests for Big numbers for varuing, varint, int and uint. In positive and negative.
// TODO: Add Null values
