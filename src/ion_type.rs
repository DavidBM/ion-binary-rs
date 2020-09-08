#[derive(Eq, PartialEq, Debug)]
pub enum ValueType {
    Null,        // T = 0   : 0000
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