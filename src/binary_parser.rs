use crate::binary_parser_types::*;
use std::fmt::Debug;
use bytes::buf::BufExt;
use std::convert::TryInto;
use std::io::Read;

pub struct IonBinaryParser<T: Read> {
    reader: T,
    current_ion_version: Option<(u8, u8)>
}

impl <T: Read>IonBinaryParser<T> {
    pub fn new(reader: T) -> IonBinaryParser<T> {
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
            return Err(ParsingError::CannotReadZeroBytes);
        }

        if octets > 8 {
            return Err(ParsingError::TooBigForU64);
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
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), ParsingError> {
        let read_bytes = self.read(buffer);

        match read_bytes {
            Ok(0) => return Err(ParsingError::NoDataToRead),
            Err(e) => return Err(ParsingError::ErrorReadingData(e.to_string())),
            Ok(len) => {
                if len < buffer.len() {
                    return Err(ParsingError::NotEnoughtDataToRead(len)),
                }

                Ok(())
            }
        }
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

    // Each byte has 7 bits forming the "VarUInt", so we cannot have more than
    // 9 segments of 7 meaningful bytes because 7 * 9 is already 63 bytes. Or we
    // can have 10 7 bites bytes, but the first one needs to be 1 or 0 (because
    // we only have one free bit in the u64).
    fn assert_valid_size_for_u64(consumed_bytes_len: usize, found_bytes: &[u8]) -> Result<(), ParsingError> {
        let max_bytes_allowed = 10;

        if consumed_bytes_len > max_bytes_allowed || 
            (consumed_bytes_len == max_bytes_allowed && (found_bytes[0] & 0b0111_1111) >= 0b_0000_0010) {
                return Err(ParsingError::TooBigForU64);
        }

        Ok(())
    }

    // Note that with this design, we can only get 62 bits for the value instead
    // of 64 as we do with the u64. This is because in 9 bytes there are 2 bites
    // for the format (end marker + sign), so in order to give the full 63 for the
    // Rust i64, we would need to add an extra bit, but because that extra bit will
    // change the layout of the second, that means that the byte would have only 0 
    // in their value part. Given that it makes things harder we won't decode that 
    // case and instead our decoder will decode only 63bits i64. Knowing this makes
    // me hope the best for who is making the ion binary encoder.
    fn assert_valid_size_for_i64(found_bytes: &[u8]) -> Result<(), ParsingError> {
        let max_bytes_allowed = 9;

        if found_bytes.len() > max_bytes_allowed {
            return Err(ParsingError::VarIntTooBigForI64);
        }

        Ok(())
    }

    //                 7  6                   0       n+7 n+6                 n
    //               +===+=====================+     +---+---------------------+
    // VarUInt field : 0 :         bits        :  …  | 1 |         bits        |
    //               +===+=====================+     +---+---------------------+
    pub fn consume_varuint(&mut self) -> Result<(u64, usize), ParsingError> {
        let found_bytes = self.consume_var_number()?;

        let consumed_bytes_len =  found_bytes.len();

        IonBinaryParser::<T>::assert_valid_size_for_u64(consumed_bytes_len, &found_bytes)?;

        let mut bytes_displacement = consumed_bytes_len - 1;
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

        Ok((final_value, consumed_bytes_len))
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

        IonBinaryParser::<T>::assert_valid_size_for_i64(&found_bytes)?;

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
                    (Ok(mut r#type), Ok(length)) => {
                        self.verify_header(&r#type, &length)?;

                        self.fill_bool_value(&mut r#type, &length)?; 

                        Ok(ValueHeader { r#type, length })
                    },
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
        }
    }

    fn fill_bool_value(&self, r#type: &mut ValueType, length: &ValueLength) -> Result<(), ParsingError> {
        match (&r#type, &length) {
            (ValueType::Bool(_), ValueLength::ShortLength(0)) => {
                *r#type = ValueType::Bool(false);
            },
            (ValueType::Bool(_), ValueLength::ShortLength(1)) => {
                *r#type = ValueType::Bool(true);
            },
            (ValueType::Bool(_), _) => {
                return Err(ParsingError::InvalidBoolLength(length.clone()));
            },
            _ => {}

        };
    
        Ok(())
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
            Bool(_) => {
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
            1 => Ok(ValueType::Bool(false)),
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

impl <T: Read> Debug for IonBinaryParser<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        fmt.debug_struct("IonBinaryParser").finish()
    }
}
