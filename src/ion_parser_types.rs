use std::collections::HashMap;
use crate::binary_parser_types::*;
use chrono::{DateTime, FixedOffset};
use crate::symbol_table::SymbolContextError;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

#[derive(Debug)]
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
} 

impl From<ParsingError> for IonParserError {
    fn from(err: ParsingError) -> Self {
        IonParserError::BinaryError(err)
    }
}

#[derive(PartialEq, Debug)]
pub enum IonValue {
    Null,
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
    Annotation((Vec<String>, Box<IonValue>)),
}

impl Eq for IonValue { }
