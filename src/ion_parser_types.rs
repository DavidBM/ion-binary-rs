use crate::binary_parser_types::*;

#[derive(Debug)]
pub enum IonParserError {
    None
} 

impl From<ParsingError> for IonParserError {
    fn from(_: ParsingError) -> Self { 
        IonParserError::None
    }
}
