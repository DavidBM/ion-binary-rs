use crate::binary_parser_types::*;
use num_bigint::{BigInt, BigUint, Sign};
use std::fmt::Debug;
use std::io::Read;
use smallvec::{SmallVec, smallvec};

pub struct IonBinaryParser<T: Read> {
    reader: T,
    current_ion_version: Option<(u8, u8)>,
}

impl<T: Read> IonBinaryParser<T> {
    #[inline]
    pub fn new(reader: T) -> IonBinaryParser<T> {
        IonBinaryParser {
            reader,
            current_ion_version: None,
        }
    }

    #[inline]
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
    #[inline]
    pub fn consume_uint(&mut self, octets: usize) -> Result<BigUint, ParsingError> {
        if octets == 0 {
            return Err(ParsingError::CannotReadZeroBytes);
        }

        let mut buffer = vec![0u8; octets];

        self.read_bytes(&mut buffer)?;

        let number = BigUint::from_bytes_be(&buffer);

        Ok(number)
    }

    #[inline]
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), ParsingError> {
        let read_bytes = self.read(buffer);

        match read_bytes {
            Ok(0) => Err(ParsingError::NoDataToRead),
            Err(e) => Err(ParsingError::ErrorReadingData(e.to_string())),
            Ok(len) => {
                if len < buffer.len() {
                    return Err(ParsingError::NotEnoughtDataToRead(len));
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
    #[inline]
    pub fn consume_int(&mut self, octets: usize) -> Result<BigInt, ParsingError> {
        if octets == 0 {
            return Err(ParsingError::CannotReadZeroBytes);
        }

        let mut buffer = vec![0u8; octets];

        self.read_bytes(&mut buffer)?;

        let is_negative = (buffer[0] & 0b1000_0000) > 0;

        buffer[0] &= 0b0111_1111;

        let mut number = BigInt::from_bytes_be(Sign::Plus, &buffer);

        if is_negative {
            number = -number;
        }

        Ok(number)
    }

    //                 7  6                   0       n+7 n+6                 n
    //               +===+=====================+     +---+---------------------+
    // VarUInt field : 0 :         bits        :  …  | 1 |         bits        |
    //               +===+=====================+     +---+---------------------+
    #[inline]
    pub fn consume_varuint(&mut self) -> Result<(BigUint, usize), ParsingError> {
        let mut bytes = self.consume_var_number()?;

        for byte in &mut bytes {
            *byte &= 0b0111_1111;
        }

        let number = match BigUint::from_radix_be(&bytes, 128) {
            Some(number) => number,
            None => return Err(ParsingError::ThisIsABugConsumingVarUInt),
        };

        Ok((number, bytes.len()))
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
    #[inline]
    pub fn consume_varint(&mut self) -> Result<(BigInt, usize), ParsingError> {
        let mut bytes = self.consume_var_number()?;

        for byte in &mut bytes {
            *byte &= 0b0111_1111;
        }

        let is_negative = bytes[0] & 0b0100_0000 > 0;

        bytes[0] &= 0b0011_1111;

        let mut number = match BigInt::from_radix_be(Sign::Plus, &bytes, 128) {
            Some(number) => number,
            None => return Err(ParsingError::ThisIsABugConsumingVarInt),
        };

        if is_negative {
            number = -number;
        }

        Ok((number, bytes.len()))
    }

    // Note: Guarantees to return at least one byte if it succeed
    #[inline]
    fn consume_var_number(&mut self) -> Result<SmallVec<[u8; 4]>, ParsingError> {
        let mut byte = [0u8; 1];

        let mut found_bytes: SmallVec<[u8; 4]> = smallvec!{};

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
    #[inline]
    pub fn consume_value_header(&mut self, nested_level: u64) -> Result<ValueHeader, ParsingError> {
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
                    if nested_level != 0 {
                        return Err(ParsingError::NestedVersionMarker);
                    }
                    let version = self.consume_ion_version_once_identified()?;
                    self.set_current_ion_version(version);
                    return self.consume_value_header(nested_level);
                }

                let value_type = (byte & 0b1111_0000) >> 4;

                let value_length = byte & 0b0000_1111;

                let value_type = self.get_field_type(value_type);
                let value_length = self.get_field_length(value_length);
                match (value_type, value_length) {
                    (Ok(r#type), Ok(length)) => {
                        self.verify_header(&r#type, &length)?;

                        Ok(ValueHeader { r#type, length })
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
        }
    }

    #[inline]
    fn verify_header(&self, valtype: &ValueType, length: &ValueLength) -> Result<(), ParsingError> {
        use ValueLength::*;
        use ValueType::*;

        match valtype {
            Annotation => {
                if let ShortLength(len) = length {
                    if len < &3 {
                        Err(ParsingError::InvalidAnnotationLength(*length))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    //                        7    0 7     0 7     0 7    0
    //                       +------+-------+-------+------+
    // binary version marker | 0xE0 | major | minor | 0xEA |
    //                       +------+-------+-------+------+
    // When calling this function we have already consumed the first byte
    // (that is how we identify we need to call this function)
    #[inline]
    fn consume_ion_version_once_identified(&mut self) -> Result<(u8, u8), ParsingError> {
        let mut byte = [0u8; 3];

        let read_bytes = self.read(&mut byte);

        match read_bytes {
            Ok(0) => Err(ParsingError::NoDataToRead),
            Err(e) => Err(ParsingError::ErrorReadingData(e.to_string())),
            Ok(_) => {
                if byte[2] != 0xEA {
                    return Err(ParsingError::BadFormedVersionHeader);
                }

                Ok((byte[0], byte[1]))
            }
        }
    }

    #[inline]
    fn set_current_ion_version(&mut self, version: (u8, u8)) {
        self.current_ion_version = Some(version);
    }

    #[inline]
    fn get_field_type(&mut self, id: u8) -> Result<ValueType, ParsingError> {
        id.try_into()
    }

    #[inline]
    fn get_field_length(&mut self, id: u8) -> Result<ValueLength, ParsingError> {
        match id {
            14 => Ok(ValueLength::LongLength), // L = 14 real length is in this field's following field: "length [VarUInt]"
            15 => Ok(ValueLength::NullValue),  // L = 15
            0..=13 => Ok(ValueLength::ShortLength(id)), // 0 <= L <= 13 and we omit the "length [VarUInt]" field,
            _ => Err(ParsingError::InvalidHeaderLength),
        }
    }
}

impl<T: Read> Debug for IonBinaryParser<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.debug_struct("IonBinaryParser").finish()
    }
}
