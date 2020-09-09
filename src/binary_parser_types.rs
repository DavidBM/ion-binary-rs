#[derive(Debug)]
pub enum SystemSymbolTableType {
	Zero,
	Ion,
	Ion1_0,
	IonSymbolTable,
	Name,
	Version,
	Imports,
	Symbols,
	MaxId,
	IonSharedSymbolTable,
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
    ErrorReadingData(String),
    CannotReadZeroBytes,
    BadFormedVersionHeader,
    InvalidNullLength(ValueLength),
    InvalidBoolLength(ValueLength),
    InvalidAnnotationLength(ValueLength)
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

