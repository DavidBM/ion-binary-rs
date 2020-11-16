use thiserror::Error;

pub const SYSTEM_SYMBOL_TABLE: &[&str; 10] = &[
    "$0",
    "$ion",
    "$ion_1_0",
    "$ion_symbol_table",
    "name",
    "version",
    "imports",
    "symbols",
    "max_id",
    "$ion_shared_symbol_table",
];

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub(crate) enum SystemSymbolIds {
    Zero = 0x00,
    Ion = 0x01,
    Ion10 = 0x02,
    IonSymbolTable = 0x03,
    Name = 0x04,
    Version = 0x05,
    Imports = 0x06,
    Symbols = 0x07,
    MaxId = 0x08,
    IonSharedSymbolTable = 0x09,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ValueLength {
    ShortLength(u8), // 0 <= L <= 13 and we omit the "length [VarUInt]" field
    LongLength,      // L = 14 real length is in this field's following field: "length [VarUInt]"
    NullValue,       // L = 15
}

#[derive(Eq, PartialEq, Debug)]
pub enum ValueType {
    Null,        // T = 0   : 0000
    Nop,         // T = 0   : 0000 (with length < 15)
    Bool,        // T = 1   : 0001
    PositiveInt, // T = 2   : 0010
    NegativeInt, // T = 3   : 0011
    Float,       // T = 4   : 0100
    Decimal,     // T = 5   : 0101
    Timestamp,   // T = 6   : 0110
    Symbol,      // T = 7   : 0111
    String,      // T = 8   : 1000
    Clob,        // T = 9   : 1001
    Blob,        // T = 10  : 1010
    List,        // T = 11  : 1011
    SExpr,       // T = 12  : 1100
    Struct,      // T = 13  : 1101
    Annotation,  // T = 14  : 1110
    Reserved,    // T = 15  : 1111
}

/// This errors indicate a problem in a primitive parsing. It comes always
/// wrapped by the "BinaryError" of the error type "IonParserError".
#[derive(Eq, PartialEq, Debug, Error)]
pub enum ParsingError {
    #[error("Header type not valid")]
    InvalidHeaderType,
    #[error("Header length not valid")]
    InvalidHeaderLength,
    #[error("Reached end of the ion stream")]
    NoDataToRead,
    #[error("There is not enough data to read, provably a premature ion stream end")]
    NotEnoughtDataToRead(usize),
    #[error("The read method returned an error which mean that the ion stream provider may have a problem")]
    ErrorReadingData(String),
    #[error("Trying to read 0 bytes")]
    CannotReadZeroBytes,
    #[error("Ion Stream Header is wrong")]
    BadFormedVersionHeader,
    #[error("Null cannot have len")]
    InvalidNullLength(ValueLength),
    #[error("Annotation cannot be shorter than 3 bytes")]
    InvalidAnnotationLength(ValueLength),
    #[error("VaruInt returned a number so huge that doesn't fit in an BitUInt")]
    ThisIsABugConsumingVarUInt,
    #[error("VaruInt returned a number so huge that doesn't fit in an BitInt")]
    ThisIsABugConsumingVarInt,
}

//   7       4 3       0
//  +---------+---------+
//  |    T    |    L    |
//  +---------+---------+
#[derive(Eq, PartialEq, Debug)]
pub struct ValueHeader {
    pub r#type: ValueType,   // T
    pub length: ValueLength, // L
}
