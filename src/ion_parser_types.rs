use std::collections::HashMap;
use crate::binary_parser_types::*;
use chrono::{DateTime, FixedOffset};
use crate::symbol_table::SymbolContextError;
use bigdecimal::BigDecimal;
#[derive(Debug)]
pub enum IonParserError {
    Unimplemented,
    BadFormatLengthFound,
    NullAnnotationFound,
    NullSymbolFound,
    SharedTableAndLocalTableDeclarationIntTheSameAnnotation,
    SymbolIdNotDefined,
    LocalTableWithoutInternalStruct,
    SharedTableDefinitionWithoutName,
    ErrorAddingSharedTableToContext(SymbolContextError),
    ErrorAddingCreatingLocal(SymbolContextError),
    LocalTableDefinitionWIthoutImportsField,
    LocalSymbolTableWithoutValidImport,
    SymbolNotFoundInTable,
    ListLengthWasTooShort,
    NonUtf8String
} 

impl From<ParsingError> for IonParserError {
    fn from(err: ParsingError) -> Self {
        println!("{:?}", err); 
        IonParserError::Unimplemented
    }
}

#[derive(PartialEq, Debug)]
pub enum IonValue {
    Null,
    Bool(bool),
    Integer(i64),
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
    Annotation((Vec<String>, Box<IonValue>))
}

impl Eq for IonValue { }
