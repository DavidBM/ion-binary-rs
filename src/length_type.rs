#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ValueLength {
    ShortLength(u8), // 0 <= L <= 13 and we omit the "length [VarUInt]" field
    LongLength, // L = 14 and the real length is in the field after this one: "length [VarUInt]"
    NullValue,  // L = 15
}