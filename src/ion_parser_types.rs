use crate::binary_parser_types::*;
use crate::symbol_table::SymbolContextError;
use bigdecimal::BigDecimal;
use chrono::{DateTime, FixedOffset};
use num_bigint::BigInt;
use std::collections::HashMap;

/// Indicated a problem in the binary blob internal structure. When all data is read
/// the library will return IonParserError::BinaryError(ParsingError::NoDataToRead).
#[derive(Eq, PartialEq, Debug)]
pub enum IonParserError {
    Unimplemented,
    BadFormatLengthFound,
    NullAnnotationFound,
    SharedTableAndLocalTableDeclarationIntTheSameAnnotation,
    SymbolIdNotDefined,
    LocalTableWithoutInternalStruct,
    SharedTableDefinitionWithoutName,
    ErrorAddingSharedTableToContext(SymbolContextError),
    ErrorAddingCreatingLocal(SymbolContextError),
    LocalSymbolTableWithoutValidImport,
    SymbolNotFoundInTable,
    ListLengthWasTooShort,
    NonUtf8String,
    DidNotGetAListConsumingAListThisIsABug,
    SymbolIdTooBig,
    TableVersionTooBig,
    DateValueTooBig,
    ValueLenTooBig,
    NotValidLengthFloat,
    BinaryError(ParsingError),
    DecimalExponentTooBig,
    InvalidBoolLength(ValueLength),
    InvalidDate(i32, u32, u32, u32, u32, u32, u32),
}

impl From<ParsingError> for IonParserError {
    fn from(err: ParsingError) -> Self {
        IonParserError::BinaryError(err)
    }
}

/// The structure wrapping all possible return ion values by the IonParser.
///
/// Please, pay attention to the Float32, Float 64 (as Ion just defined "float")
/// and the Integer and BigInteger. The parser will return the most adequate
/// Integer type. If you expect small numbers you can get by with Integer alone,
/// but if you don't know, you will need to match both types.
#[derive(PartialEq, Debug, Clone)]
pub enum IonValue {
    Null(NullIonValue),
    Bool(bool),
    Integer(i64),
    BigInteger(BigInt),
    Float32(f32),
    Float64(f64),
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
#[derive(PartialEq, Debug, Clone)]
pub enum NullIonValue {
    Null,
    Bool,
    Integer,
    BigInteger,
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
