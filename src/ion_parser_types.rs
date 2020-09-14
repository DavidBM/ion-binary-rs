use std::collections::HashMap;
use crate::binary_parser_types::*;
use chrono::{DateTime, Utc};
use crate::symbol_table::SymbolContextError;

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
    ListLengthWasTooShort
} 

impl From<ParsingError> for IonParserError {
    fn from(_: ParsingError) -> Self { 
        IonParserError::Unimplemented
    }
}

#[derive(PartialEq, Debug)]
pub enum IonValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Decimal((u64, i64)),
    Timestamp(DateTime<Utc>),
    String(String),
    Symbol(String),
    Clob(Vec<u8>),
    Blob(Vec<u8>),
    List(Vec<IonValue>), 
    SExpr(Vec<IonValue>),
    Struct(HashMap<String, IonValue>),
    Annotation((Vec<String>, Box<IonValue>))
}
