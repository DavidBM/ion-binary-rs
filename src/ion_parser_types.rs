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
    NestedAnnotations,
    BadAnnotationLength,
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
    DateSecondFractionOverflow,
    DateNegativeSecondFraction,
    DateWithHourButNoMinutes,
    ValueLenTooBig,
    NotValidLengthFloat,
    BinaryError(ParsingError),
    DecimalExponentTooBig,
    InvalidBoolLength(ValueLength),
    InvalidDate(i32, u32, u32, u32, u32, u32, u32),
    InvalidReservedTypeDescriptor,
    InvalidNegativeInt,
    EmptyOrderedStruct,
}

impl From<ParsingError> for IonParserError {
    fn from(err: ParsingError) -> Self {
        IonParserError::BinaryError(err)
    }
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
#[derive(PartialEq, Debug, Clone)]
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
