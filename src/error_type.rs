use crate::length_type::ValueLength;

#[derive(Eq, PartialEq, Debug)]
pub enum ParsingError {
    InvalidHeaderType,
    InvalidHeaderLength,
    TooBigForU64,
    VarIntTooBigForI64,
    NoDataToRead,
    ErrorReadingData(String),
    CannotReadZeroBytes,
    BadFormedVersionHeader,
    InvalidNullLength(ValueLength),
    InvalidBoolLength(ValueLength),
    InvalidAnnotationLength(ValueLength)
}