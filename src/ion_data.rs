//! The full Ion 1.0 data model from the official `ion-rs` crate.
//!
//! ```
//! use ion_binary_rs::ion_data::{from_binary, to_binary, Element, IonData, IonResult};
//!
//! # fn main() -> IonResult<()> {
//! let values = Element::read_all(r#"{item: 2024T, item: -0.00}"#)?;
//! let binary = to_binary(&values)?;
//! let decoded = from_binary(&binary)?;
//! assert_eq!(IonData::from(decoded), IonData::from(values));
//! # Ok(())
//! # }
//! ```

use ion_rs::v1_0::Binary;
use std::io;

pub use ion_rs::{
    Annotations, Decimal, Element, Int, IonData, IonError, IonResult, IonType, Sequence, Struct,
    Symbol, Timestamp, TimestampPrecision, Value,
};

/// Reads an Ion 1.0 binary stream, preserving information such as repeated field
/// names, time precision, unknown symbol text, and the sign of decimal zero.
pub fn from_binary(bytes: &[u8]) -> IonResult<Sequence> {
    if !bytes.starts_with(&[0xE0, 1, 0, 0xEA]) {
        return Err(IonError::application(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected an Ion 1.0 binary version marker",
        )));
    }
    Element::read_all(bytes)
}

/// Writes the elements as one Ion 1.0 binary stream.
pub fn to_binary(values: &Sequence) -> IonResult<Vec<u8>> {
    values.encode_as(Binary)
}
