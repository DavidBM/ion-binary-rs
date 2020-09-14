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
    LongLength, // L = 14 and the real length is in the field after this one: "length [VarUInt]"
    NullValue,  // L = 15
}

#[derive(Eq, PartialEq, Debug)]
pub enum ValueType {
    Null,        // T = 0   : 0000
    Bool(bool),  // T = 1   : 0001
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

#[derive(Eq, PartialEq, Debug)]
pub enum ParsingError {
    InvalidHeaderType,
    InvalidHeaderLength,
    TooBigForU64,
    VarIntTooBigForI64,
    NoDataToRead,
    NotEnoughtDataToRead(usize),
    ErrorReadingData(String),
    CannotReadZeroBytes,
    BadFormedVersionHeader,
    InvalidNullLength(ValueLength),
    InvalidBoolLength(ValueLength),
    InvalidAnnotationLength(ValueLength),
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

