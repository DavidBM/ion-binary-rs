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
    Zero = 0x0,
    Ion = 0x1,
    Ion10 = 0x2,
    IonSymbolTable = 0x3,
    Name = 0x4,
    Version = 0x5,
    Imports = 0x6,
    Symbols = 0x7,
    MaxId = 0x8,
    IonSharedSymbolTable = 0x9,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ValueLength {
    ShortLength(u8), // 0 <= L <= 13 and we omit the "length [VarUInt]" field
    LongLength,      // L = 14 real length is in this field's following field: "length [VarUInt]"
    NullValue,       // L = 15
}

impl From<ValueLength> for u8 {
    fn from(input: ValueLength) -> u8 {
        match input {
            ValueLength::ShortLength(len) => len,
            ValueLength::LongLength => 14,
            ValueLength::NullValue => 15,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ValueType {
    Null = 0x0,        // T = 0   : 0000 || NOP -> T = 0   : 0000 (with length < 15)
    Bool = 0x1,        // T = 1   : 0001
    PositiveInt = 0x2, // T = 2   : 0010
    NegativeInt = 0x3, // T = 3   : 0011
    Float = 0x4,       // T = 4   : 0100
    Decimal = 0x5,     // T = 5   : 0101
    Timestamp = 0x6,   // T = 6   : 0110
    Symbol = 0x7,      // T = 7   : 0111
    String = 0x8,      // T = 8   : 1000
    Clob = 0x9,        // T = 9   : 1001
    Blob = 0xa,        // T = 10  : 1010
    List = 0xb,        // T = 11  : 1011
    SExpr = 0xc,       // T = 12  : 1100
    Struct = 0xd,      // T = 13  : 1101
    Annotation = 0xe,  // T = 14  : 1110
    Reserved = 0xf,    // T = 15  : 1111
}

impl TryFrom<u8> for ValueType {
    type Error = ParsingError;

    fn try_from(id: u8) -> Result<Self, ParsingError> {
        match id {
            0 => Ok(ValueType::Null),
            1 => Ok(ValueType::Bool),
            2 => Ok(ValueType::PositiveInt),
            3 => Ok(ValueType::NegativeInt),
            4 => Ok(ValueType::Float),
            5 => Ok(ValueType::Decimal),
            6 => Ok(ValueType::Timestamp),
            7 => Ok(ValueType::Symbol),
            8 => Ok(ValueType::String),
            9 => Ok(ValueType::Clob),
            10 => Ok(ValueType::Blob),
            11 => Ok(ValueType::List),
            12 => Ok(ValueType::SExpr),
            13 => Ok(ValueType::Struct),
            14 => Ok(ValueType::Annotation),
            15 => Ok(ValueType::Reserved),
            _ => Err(ParsingError::InvalidHeaderType),
        }
    }
}

/// This errors indicate a problem in a primitive parsing. It comes always
/// wrapped by the "BinaryError" of the error type "IonParserError".
#[derive(Debug, Error)]
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
    ErrorReadingData(#[from] std::io::Error),
    #[error("Trying to read 0 bytes")]
    CannotReadZeroBytes,
    #[error("Ion Stream Header is wrong")]
    BadFormedVersionHeader,
    #[error("Null cannot have len")]
    InvalidNullLength(u8),
    #[error("Annotation cannot be shorter than 3 bytes")]
    InvalidAnnotationLength(u8),
    #[error("VaruInt returned a number so huge that doesn't fit in an BitUInt")]
    ThisIsABugConsumingVarUInt,
    #[error("VaruInt returned a number so huge that doesn't fit in an BitInt")]
    ThisIsABugConsumingVarInt,
    #[error("An Ion version markers was found in a nested structure")]
    NestedVersionMarker,
}

impl PartialEq for ParsingError {
    fn eq(&self, input: &ParsingError) -> bool {
        match (self, input) {
            (ParsingError::ErrorReadingData(a), ParsingError::ErrorReadingData(b))
                if a.kind() == b.kind() =>
            {
                true
            }
            (a, b) => a == b,
        }
    }
}

//   7       4 3       0
//  +---------+---------+
//  |    T    |    L    |
//  +---------+---------+
#[derive(Eq, PartialEq, Debug)]
pub struct ValueHeader(u8);

impl ValueHeader {
    #[inline]
    pub fn new(byte: u8) -> Result<ValueHeader, ParsingError> {
        let header = ValueHeader(byte);

        if header.get_type() == ValueType::Annotation && header.get_len() < 3 {
            Err(ParsingError::InvalidAnnotationLength(header.get_len()))
        } else {
            Ok(header)
        }
    }

    pub fn is_nop(&self) -> bool {
        let ion_type = self.get_type();

        ion_type == ValueType::Null && self.get_len() < 15
    }

    pub fn get_type(&self) -> ValueType {
        ValueHeader::from_safe_u8(self.0 >> 4)
    }

    pub fn get_len(&self) -> u8 {
        self.0 & 0b0000_1111
    }

    pub fn is_len_null_value(&self) -> bool {
        (self.0 & 0b0000_1111) == 15
    }

    pub fn is_len_long_len(&self) -> bool {
        (self.0 & 0b0000_1111) == 14
    }

    // pub fn is_len_short_len(&self) -> bool {
    //     (self.0 & 0b0000_1111) < 14
    // }

    fn from_safe_u8(id: u8) -> ValueType {
        match id {
            0 => ValueType::Null,
            1 => ValueType::Bool,
            2 => ValueType::PositiveInt,
            3 => ValueType::NegativeInt,
            4 => ValueType::Float,
            5 => ValueType::Decimal,
            6 => ValueType::Timestamp,
            7 => ValueType::Symbol,
            8 => ValueType::String,
            9 => ValueType::Clob,
            10 => ValueType::Blob,
            11 => ValueType::List,
            12 => ValueType::SExpr,
            13 => ValueType::Struct,
            14 => ValueType::Annotation,
            15 => ValueType::Reserved,
            _ => panic!("Internal library bug"),
        }
    }
}
