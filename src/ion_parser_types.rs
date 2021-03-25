use crate::binary_parser_types::*;
use crate::symbol_table::SymbolContextError;
use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;

/// Indicated a problem in the binary blob internal structure. When all data is read
/// the library will return IonParserError::BinaryError(ParsingError::NoDataToRead).
#[derive(PartialEq, Debug, Error)]
pub enum IonParserError {
    #[error("Ion type not implemented")]
    Unimplemented,
    #[error("Null annotation found")]
    NullAnnotationFound,
    #[error("Nested annotation found")]
    NestedAnnotations,
    #[error("Annotation len field is wrong")]
    BadAnnotationLength,
    #[error("Ion symbol table declared as local and shared at the same time")]
    SharedTableAndLocalTableDeclarationIntTheSameAnnotation,
    #[error("Not known symbol id")]
    SymbolIdNotDefined,
    #[error("Local table structure definition is not correct")]
    LocalTableWithoutInternalStruct,
    #[error("The shared table is missing it's name")]
    SharedTableDefinitionWithoutName,
    #[error("Error adding the shared table into the context")]
    ErrorAddingSharedTableToContext(SymbolContextError),
    #[error("Error importing local table")]
    ErrorAddingCreatingLocal(SymbolContextError),
    #[error("The local table don't have a valid import")]
    LocalSymbolTableWithoutValidImport,
    #[error("Symbol id not found")]
    SymbolNotFoundInTable,
    #[error("The list content is longer than what it's len field indicates")]
    ListLengthWasTooShort,
    #[error("Ion String is not valid UTF-8")]
    NonUtf8String,
    #[error("Internal error: Open a bug on github")]
    DidNotGetAListConsumingAListThisIsABug,
    #[error("Symbol id is bigger than usize")]
    SymbolIdTooBig,
    #[error("Table version bigger than usize")]
    TableVersionTooBig,
    #[error("Date value is far too big for a date")]
    DateValueTooBig,
    #[error("The second fraction is bigger than 1 second")]
    DateSecondFractionOverflow,
    #[error("The second fraction is negative")]
    DateNegativeSecondFraction,
    #[error("The date has hours defined but not minutes, which is illegal")]
    DateWithHourButNoMinutes,
    #[error("The length of the value is bigger than usize, which is far too")]
    ValueLenTooBig,
    #[error("Floats can only be 4 or 8 bytes")]
    NotValidLengthFloat,
    #[error("Error parsing the ion binary format")]
    BinaryError(ParsingError),
    #[error("Exponent for the decimal value is too big (greater than i64)")]
    DecimalExponentTooBig,
    #[error("Bool cannot have len")]
    InvalidBoolLength(ValueLength),
    #[error("The date is not valid")]
    InvalidDate(i32, u32, u32, u32, u32, u32, u32),
    #[error("Ion type 15 doesn't exist in Ion 1.0")]
    InvalidReservedTypeDescriptor,
    #[error("Negative ints need a value")]
    InvalidNegativeInt,
    #[error("Ordered structs cannot be empty")]
    EmptyOrderedStruct,
    #[error("Error transforming the IonValue to a rust type")]
    ValueExtractionFailure(IonExtractionError),
}

impl From<ParsingError> for IonParserError {
    fn from(err: ParsingError) -> Self {
        IonParserError::BinaryError(err)
    }
}

#[derive(Debug, Error)]
pub enum IonExtractionError {
    #[error("The current type doesn't support the requested transformation")]
    TypeNotSupported(IonValue),
    #[error("The current type doesn't support the requested transformation")]
    NumericTransformationError(Box<dyn Error + Send + Sync>),
}

impl PartialEq for IonExtractionError {
    fn eq(&self, other: &IonExtractionError) -> bool {
        use IonExtractionError::NumericTransformationError;

        match (self, other) {
            (NumericTransformationError(err_a), NumericTransformationError(err_b)) => {
                format!("{}", err_a) == format!("{}", err_b)
            }
            _ => *self == *other,
        }
    }
}

#[derive(PartialEq, Debug, Error)]
pub enum SerdeJsonParseError {
    #[error("Library tells a wrong number type")]
    WrongNumberType,
    #[error("Non existent number type")]
    NonExistentNumberType,
}

/// The structure wrapping all possible return ion values by the IonParser.
///
/// Please, pay attention to Integer and BigInteger. The parser will return the
/// most adequate integer type. If you expect small numbers you can get by with
/// Integer alone, but if you don't know, you will need to match both types.
///
/// Floats are implemented only using f64. Previously there was Float32 and
/// Float64, but there are some problems with IonHash and QLDB when using Float32.
#[derive(PartialEq, Debug, Clone)]
pub enum IonValue {
    Null(NullIonValue),
    Bool(bool),
    Integer(i64),
    BigInteger(BigInt),
    Float(f64),
    Decimal(BigDecimal),
    DateTime(DateTime<FixedOffset>),
    String(String),
    Symbol(String),
    Clob(Vec<u8>),
    Blob(Vec<u8>),
    List(Vec<IonValue>),
    SExpr(Vec<IonValue>),
    Struct(HashMap<String, IonValue>),
    Annotation(Vec<String>, Box<IonValue>),
}

impl Eq for IonValue {}

/// Instead of wrapping each IonValue in an Option in order to represent the
/// null value, we opted to join all Null values in the IonValue::Null(_) which
/// contains this struct. Here you can check what kind of null you got. We do this
/// because we believe is more ergonomic and simplifies the API handling.
#[derive(PartialEq, Debug, Clone, Eq)]
pub enum NullIonValue {
    Null,
    Bool,
    Integer,
    Float,
    Decimal,
    DateTime,
    String,
    Symbol,
    Clob,
    Blob,
    List,
    SExpr,
    Struct,
    Annotation,
}
