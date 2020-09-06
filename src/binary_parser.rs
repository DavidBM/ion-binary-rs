use std::fmt::Debug;
use bytes::buf::BufExt;
use std::convert::TryInto;
use std::io::Read;

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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ValueLength {
    ShortLength(u8), // 0 <= L <= 13 and we omit the "length [VarUInt]" field
    LongLength, // L = 14 and the real length is in the field after this one: "length [VarUInt]"
    NullValue,  // L = 15
}

//   7       4 3       0
//  +---------+---------+
//  |    T    |    L    |
//  +---------+---------+
#[derive(Eq, PartialEq, Debug)]
pub struct ValueHeader {
    r#type: ValueType,   // T
    length: ValueLength, // L
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

pub struct IonBinaryParser {
    reader: Box<dyn Read>,
    current_ion_version: Option<(u8, u8)>
}

impl IonBinaryParser {
    pub fn new(reader: Box<dyn Read>) -> IonBinaryParser {
        IonBinaryParser { reader, current_ion_version: None }
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.reader.read(buffer)
    }

    //             7                       0
    //            +-------------------------+
    // UInt field |          bits           |
    //            +-------------------------+
    //            :          bits           :
    //            +=========================+
    //                        ⋮
    //            +=========================+
    //            :          bits           :
    //            +=========================+
    //             n+7                     n
    pub fn consume_uint(&mut self, octets: usize) -> Result<u64, ParsingError> {
        if octets == 0 {
            return Err(ParsingError::CannotReadZeroBytes)
        }

        if octets > 8 {
            return Err(ParsingError::TooBigForU64)
        }

        let mut byte = [0u8; 1];

        let mut final_value = 0_u64;

        for displacement in 0..octets {
            let displacement = octets - 1 - displacement;

            let read_bytes = self.read(&mut byte);

            match read_bytes {
                Ok(0) => return Err(ParsingError::NoDataToRead),
                Err(e) => return Err(ParsingError::ErrorReadingData(e.to_string())),
                Ok(_) => {
                    let mut temporal_value: u64 = byte[0].into();
                    temporal_value <<= displacement * 8;
                    final_value |= temporal_value;
                }
            }

        }

        Ok(final_value)
    }

    //              7  6                   0
    //            +---+---------------------+
    // Int field  |   |      bits           |
    //            +---+---------------------+
    //              ^
    //              |
    //              +--sign
    //            +=========================+
    //            :          bits           :
    //            +=========================+
    //                        ⋮
    //            +=========================+
    //            :          bits           :
    //            +=========================+
    //             n+7                     n
    pub fn consume_int(&mut self, octets: usize) -> Result<i64, ParsingError> {
        if octets == 0 {
            return Err(ParsingError::CannotReadZeroBytes)
        }

        if octets > 8 {
            return Err(ParsingError::TooBigForU64)
        }

        let mut byte = [0u8; 1];


        let read_bytes = self.read(&mut byte);

        if let Ok(0) = read_bytes {
            return Err(ParsingError::NoDataToRead);
        }

        if let Err(e) = read_bytes {
            return Err(ParsingError::ErrorReadingData(e.to_string()));
        }

        let is_negative = (byte[0] & 0b1000_0000) > 0;

        let mut final_value: u64 = (byte[0] & 0b0111_1111).into();
        final_value <<= (octets - 1) * 8;

        for displacement in 1..octets {
            let displacement = octets - 1 - displacement;

            let read_bytes = self.read(&mut byte);

            match read_bytes {
                Ok(0) => return Err(ParsingError::NoDataToRead),
                Err(e) => return Err(ParsingError::ErrorReadingData(e.to_string())),
                Ok(_) => {
                    let mut temporal_value: u64 = byte[0].into();
                    temporal_value <<= displacement * 8;
                    final_value |= temporal_value;
                }
            }

        }

        // If this doesn't work we want to fail as that should be impossible
        let mut final_value: i64 = final_value.try_into().unwrap();

        if is_negative {
            final_value = -final_value;
        }

        Ok(final_value)
    }

    //                 7  6                   0       n+7 n+6                 n
    //               +===+=====================+     +---+---------------------+
    // VarUInt field : 0 :         bits        :  …  | 1 |         bits        |
    //               +===+=====================+     +---+---------------------+
    pub fn consume_varuint(&mut self) -> Result<u64, ParsingError> {
        let found_bytes = self.consume_var_number()?;

        // Each byte has 7 bits forming the "VarUInt", so we cannot have more than
        // 9 segments of 7 meaningful bytes because 7 * 9 is already 63 bytes. Or we
        // can have 10 7 bites bytes, but the first one needs to be 1 or 0 (because
        // we only have one free bite in the u64).
        if found_bytes.len() > 10
            || (found_bytes.len() == 10 && (found_bytes[0] & 0b0111_1111) >= 0b_0000_0010)
        {
            return Err(ParsingError::TooBigForU64);
        }

        let mut bytes_displacement = found_bytes.len() - 1;
        let mut final_value = 0_u64;

        for byte in found_bytes {
            let byte = byte & 0b_0111_1111;

            let mut value_buffer: u64 = byte.into();
            value_buffer <<= bytes_displacement * 7;

            if bytes_displacement > 0 {
                bytes_displacement -= 1;
            }

            final_value |= value_buffer;

            value_buffer = 0;
        }

        Ok(final_value)
    }

    //                7   6  5               0       n+7 n+6                 n
    //              +===+                           +---+
    // VarInt field : 0 :       payload          …  | 1 |       payload
    //              +===+                           +---+
    //                  +---+-----------------+         +=====================+
    //                  |   |   magnitude     |  …      :       magnitude     :
    //                  +---+-----------------+         +=====================+
    //                ^   ^                           ^
    //                |   |                           |
    //                |   +--sign                     +--end flag
    //                +--end flag
    //
    //                             7   6  5           0
    //                           +---+---+-------------+
    // single octet VarInt field | 1 |   |  magnitude  |
    //                           +---+---+-------------+
    //                                 ^
    //                                 |
    //                                 +--sign
    pub fn consume_varint(&mut self) -> Result<i64, ParsingError> {
        let found_bytes = self.consume_var_number()?;

        // Note that with this design, we can only get 62 bits for the value instead
        // of 64 as we do with the u64. This is because in 9 bytes there are 2 bites
        // for the format (end marker + sign), so in order to give the full 63 for the
        // Rust i64, we would need to add an extra bit, but because that extra bit will
        // change the layout of the second, that means that the byte would have only 0 
        // in their value part. Given that it makes things harder we won't decode that 
        // case and instead our decoder will decode only 63bits i64. Knowing this makes
        // me hope the best for who is making the ion binary encoder.
        if found_bytes.len() > 9 {
            return Err(ParsingError::VarIntTooBigForI64);
        }

        // consume_var_number function is guaranteed to return at least one byte.
        let is_negative = (found_bytes[0] & 0b0100_0000) > 0;
        // How many iteration we will do. If only one byte, 0 iterations in the while loop
        let mut bytes_displacement = found_bytes.len() - 1;

        // We ignore the first bit and the second (sign bit)
        let mut final_value: u64 = ((found_bytes[0] & 0b0011_1111) as u8).into();

        final_value <<= bytes_displacement * 7;

        //ignore the first byte, as it has already been processed
        for byte in &found_bytes[1..] {
            bytes_displacement -= 1;

            let current_byte_value: u8 = byte & 0b0111_1111;

            let mut temporal_value: u64 = current_byte_value.into();

            temporal_value <<= bytes_displacement * 7;

            final_value |= temporal_value;
        }

        // If this doesn't work we want to fail as that should be impossible 
        let mut final_value: i64 = final_value.try_into().unwrap();

        if is_negative {
            final_value = -final_value;
        }

        Ok(final_value)
    }

    // Note: Guarantees to return at least one byte if it succeed
    fn consume_var_number(&mut self) -> Result<Vec<u8>, ParsingError> {
        let mut byte = [0u8; 1];

        let mut found_bytes: Vec<u8> = vec![];

        loop {
            let read_bytes = self.read(&mut byte);

            match read_bytes {
                Ok(0) => return Err(ParsingError::NoDataToRead),
                Err(e) => return Err(ParsingError::ErrorReadingData(e.to_string())),
                Ok(_) => {
                    found_bytes.push(byte[0]);

                    // Last byte is marked with a 1 in the highest bite
                    if 0b1000_0000 & byte[0] == 0b1000_0000 {
                        break;
                    }
                }
            }
        }

        Ok(found_bytes)
    }

    //   7       4 3       0
    //  +---------+---------+
    //  |    T    |    L    |
    //  +---------+---------+
    pub fn consume_value_header(&mut self) -> Result<ValueHeader, ParsingError> {
        let mut byte = [0u8; 1];

        let read_bytes = self.read(&mut byte);

        match read_bytes {
            Ok(0) => Err(ParsingError::NoDataToRead),
            Err(e) => Err(ParsingError::ErrorReadingData(e.to_string())),
            Ok(_) => {

                let byte = byte[0];

                // If the byte has T as E (annotation) with a L of 0 (invalid) 
                // it means that this is a ion version header, so we read it
                // and set the decoder to the new version. 
                if byte == 0xE0 {
                    let version = self.consume_ion_version_once_identified()?;
                    self.set_current_ion_version(version);
                    return self.consume_value_header();
                }

                let value_type = (byte & 0b1111_0000) >> 4;

                let value_length = byte & 0b0000_1111;

                let value_type = self.get_field_type(value_type);
                let value_length = self.get_field_length(value_length);

                match (value_type, value_length) {
                    (Ok(r#type), Ok(length)) => {
                        self.verify_header(&r#type, &length)?;
                        Ok(ValueHeader { r#type, length })
                    },
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
        }
    }

    fn verify_header(&self, valtype: &ValueType, length: &ValueLength) -> Result<(), ParsingError> {
        use ValueType::*;
        use ValueLength::*;

        match valtype {
            Null => {
                if let NullValue = length {
                    Ok(())
                } else {
                    Err(ParsingError::InvalidNullLength(length.clone()))
                }
            },
            Bool => {
                if let ShortLength(len) = length {
                    if len > &1 {
                        Err(ParsingError::InvalidBoolLength(length.clone()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(ParsingError::InvalidBoolLength(length.clone()))
                }
            },
            Annotation => {
                if let ShortLength(len) = length {
                    if len < &3 {
                        Err(ParsingError::InvalidAnnotationLength(length.clone()))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
            _ => Ok(())
        }
    }

    //                        7    0 7     0 7     0 7    0
    //                       +------+-------+-------+------+
    // binary version marker | 0xE0 | major | minor | 0xEA |
    //                       +------+-------+-------+------+
    // When calling this function we have already consumed the first byte 
    // (that is how we identify we need to call this function)
    fn consume_ion_version_once_identified(&mut self) -> Result<(u8, u8), ParsingError>{
        let mut byte = [0u8; 3];

        let read_bytes = self.read(&mut byte);

        match read_bytes {
            Ok(0) => Err(ParsingError::NoDataToRead),
            Err(e) => Err(ParsingError::ErrorReadingData(e.to_string())),
            Ok(_) => {
                if byte[2] != 0xEA {
                    return Err(ParsingError::BadFormedVersionHeader);
                }

                Ok( (byte[0], byte[1]) )
            }
        }
    }

    fn set_current_ion_version(&mut self, version: (u8, u8)) {
        self.current_ion_version = Some(version);
    }

    pub fn get_current_ion_version(&self) -> Option<(u8, u8)>{
        self.current_ion_version
    }

    fn get_field_type(&mut self, id: u8) -> Result<ValueType, ParsingError> {
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

    fn get_field_length(&mut self, id: u8) -> Result<ValueLength, ParsingError> {
        match id {
            14 => Ok(ValueLength::LongLength), // L = 14 and the real length is in the field after this one: "length [VarUInt]"
            15 => Ok(ValueLength::NullValue),  // L = 15
            0..=13 => Ok(ValueLength::ShortLength(id)), // 0 <= L <= 13 and we omit the "length [VarUInt]" field,
            _ => Err(ParsingError::InvalidHeaderLength),
        }
    }
}

impl Debug for IonBinaryParser {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        fmt.debug_struct("IonBinaryParser").finish()
    }
}

#[test]
fn decode_value_null() {
    let ion_test = [0b_0000_1111u8].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_value_header(),
        Ok(ValueHeader {
            r#type: ValueType::Null,
            length: ValueLength::NullValue,
        })
    );
}

#[test]
fn decode_value_invalid_null() {
    let ion_test = [0b_0000_1110u8].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_value_header(),
        Err(ParsingError::InvalidNullLength(ValueLength::LongLength))
    );
}

#[test]
fn decode_varuint_one_byte() {
    let ion_test = [0b_1000_1000u8].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varuint(), Ok(8));
}

#[test]
fn decode_varuint_two_byte_only_last_byte_significant() {
    let ion_test = [0b_0000_0000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varuint(), Ok(8));
}

#[test]
fn decode_varuint_two_byte() {
    let ion_test = [0b_0001_0000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varuint(), Ok(2056));
}

#[test]
fn decode_varuint_three_byte() {
    let ion_test = [0b_0001_0000, 0b_0000_1000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varuint(), Ok(263176));
}

#[test]
fn decode_varuint_len_10() {
    let ion_test = [
        0b_0000_0001u8,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_1000_0000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varuint(),
        Ok(9804371850199958528)
    );
}

#[test]
fn decode_varuint_too_long_len_10() {
    let ion_test = [
        0b_0000_0010,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varuint(),
        Err(ParsingError::TooBigForU64)
    );
}

#[test]
fn decode_varuint_too_long_len_11() {
    let ion_test = [
        0b_0001_0000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varuint(),
        Err(ParsingError::TooBigForU64)
    );
}

#[test]
fn decode_varint_one_byte_negative() {
    let ion_test = [0b_1100_1000u8].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(-8));
}

#[test]
fn decode_varint_one_byte_positive() {
    let ion_test = [0b_1000_1000u8].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(8));
}

#[test]
fn decode_varint_two_byte_only_last_byte_significant_negative() {
    let ion_test = [0b_0100_0000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(-8));
}

#[test]
fn decode_varint_two_byte_only_last_byte_significant_positive() {
    let ion_test = [0b_0000_0000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(8));
}

#[test]
fn decode_varint_two_byte_positive() {
    let ion_test = [0b_0001_0000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(2056));
}

#[test]
fn decode_varint_two_byte_negative() {
    let ion_test = [0b_0101_0000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(-2056));
}

#[test]
fn decode_varint_three_byte_positive() {
    let ion_test = [0b_0001_0000, 0b_0000_1000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(263176));
}

#[test]
fn decode_varint_three_byte_negative() {
    let ion_test = [0b_0101_0000, 0b_0000_1000, 0b_1000_1000].reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(lexer.consume_varint(), Ok(-263176));
}

#[test]
fn decode_varint_len_10_positive() {
    let ion_test = [
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varint(),
        Ok(580999813345182728)
    );
}

#[test]
// Technically correct, but we don't handle this case (yet?) 
fn decode_varint_valid_but_not_handles_case_len_10_positive() {
    let ion_test = [
        0b_0000_0000,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_1111_1111,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varint(),
        Err(ParsingError::VarIntTooBigForI64)
    );
}

#[test]
// Technically correct, but we don't handle this case (yet?) 
fn decode_varint_valid_but_not_handles_case_len_10_negative() {
    let ion_test = [
        0b_0100_0000,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_1111_1111,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varint(),
        Err(ParsingError::VarIntTooBigForI64)
    );
}

#[test]
// Technically correct, but we don't handle this case (yet?) 
fn decode_varint_len_10_max_positive() {
    let ion_test = [
        0b_0011_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_1111_1111,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varint(),
        Ok(4611686018427387903)
    );
}

#[test]
// Technically correct, but we don't handle this case (yet?) 
fn decode_varint_len_10_max_negative() {
    let ion_test = [
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_0111_1111,
        0b_1111_1111,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_varint(),
        Ok(-4611686018427387903)
    );
}

#[test]
fn decode_uint_valid_len_8() {
    let ion_test = [
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_uint(8),
        Ok(8)
    );
}

#[test]
fn decode_uint_valid() {
    let ion_test = [
        0b_0000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_uint(1),
        Ok(8)
    );
}

#[test]
fn decode_uint_valid_2() {
    let ion_test = [
        0b_0000_1000,
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_uint(2),
        Ok(2184)
    );
}

#[test]
fn decode_uint_invalid_zero_len() {
    let ion_test = [
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_uint(0),
        Err(ParsingError::CannotReadZeroBytes)
    );
}

#[test]
fn decode_int_valid_len_8_positive() {
    let ion_test = [
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(8),
        Ok(8)
    );
}

#[test]
fn decode_int_valid_len_8_negative() {
    let ion_test = [
        0b_1000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(8),
        Ok(-8)
    );
}

#[test]
fn decode_int_valid_positive() {
    let ion_test = [
        0b_0000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(1),
        Ok(8)
    );
}

#[test]
fn decode_int_valid_negative() {
    let ion_test = [
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(1),
        Ok(-8)
    );
}

#[test]
fn decode_int_valid_2_positive() {
    let ion_test = [
        0b_0000_1000,
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(2),
        Ok(2184)
    );
}

#[test]
fn decode_int_valid_2_negative() {
    let ion_test = [
        0b_1000_1000,
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(2),
        Ok(-2184)
    );
}

#[test]
fn decode_int_invalid_zero_len() {
    let ion_test = [
        0b_1000_1000,
    ]
    .reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_int(0),
        Err(ParsingError::CannotReadZeroBytes)
    );
}

#[test]
fn decode_value_with_version_header() {
    let ion_test = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83V".reader();

    let mut lexer = IonBinaryParser::new(Box::new(ion_test));

    assert_eq!(
        lexer.consume_value_header(),
        Ok(ValueHeader { 
            r#type: ValueType::Annotation,
            length: ValueLength::LongLength,
        })
    );
}

#[test]
fn decode_full_ion() {
    let _ion = b"\xe0\x01\0\xea\xee\xa6\x81\x83\xde\xa2\x87\xbe\x9f\x83VIN\x84Type\x84Year\x84Make\x85Model\x85Color\xde\xb9\x8a\x8e\x911C4RJFAG0FC625797\x8b\x85Sedan\x8c\"\x07\xe3\x8d\x88Mercedes\x8e\x87CLK 350\x8f\x85White";
}
