use crate::binary_parser_types::*;
use num_bigint::{BigInt, BigUint, Sign};
use std::fmt::Debug;
use std::io::{self, Read};

pub struct IonBinaryParser<T: Read> {
    reader: T,
    symbol_reset_pending: bool,
    temp_buffer: Vec<u8>,
}

impl<T: Read> IonBinaryParser<T> {
    #[inline]
    pub fn new(reader: T) -> IonBinaryParser<T> {
        IonBinaryParser {
            reader,
            symbol_reset_pending: false,
            temp_buffer: Vec::with_capacity(256),
        }
    }

    fn read_byte(&mut self) -> Result<u8, ParsingError> {
        let mut byte = [0];
        Self::read_bytes_no_self(&mut self.reader, &mut byte)?;
        Ok(byte[0])
    }

    pub fn take_symbol_reset(&mut self) -> bool {
        std::mem::take(&mut self.symbol_reset_pending)
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

        Self::read_to_buffer(&mut self.reader, &mut self.temp_buffer, octets)?;

        let number = BigUint::from_bytes_be(&self.temp_buffer);

        Ok(number)
    }

    #[inline]
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), ParsingError> {
        IonBinaryParser::read_bytes_no_self(&mut self.reader, buffer)
    }

    fn read_bytes_no_self(reader: &mut T, buffer: &mut [u8]) -> Result<(), ParsingError> {
        let mut read = 0;
        while read < buffer.len() {
            match reader.read(&mut buffer[read..]) {
                Ok(0) => return Err(Self::incomplete_read(read)),
                Ok(len) => read += len,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }

    fn incomplete_read(read: usize) -> ParsingError {
        match read {
            0 => ParsingError::NoDataToRead,
            read => ParsingError::NotEnoughtDataToRead(read),
        }
    }

    fn read_to_buffer(
        reader: &mut T,
        buffer: &mut Vec<u8>,
        length: usize,
    ) -> Result<(), ParsingError> {
        buffer.clear();
        if length <= 256 {
            buffer.resize(length, 0);
            return Self::read_bytes_no_self(reader, buffer);
        }

        // Grow only as the bytes arrive. The length comes from the input stream.
        let read = reader.take(length as u64).read_to_end(buffer)?;
        if read != length {
            return Err(Self::incomplete_read(read));
        }
        Ok(())
    }

    pub fn read_value_bytes(&mut self, length: usize) -> Result<Vec<u8>, ParsingError> {
        let mut buffer = Vec::new();
        Self::read_to_buffer(&mut self.reader, &mut buffer, length)?;
        Ok(buffer)
    }

    pub fn skip_bytes(&mut self, length: usize) -> Result<(), ParsingError> {
        let read = io::copy(
            &mut self.reader.by_ref().take(length as u64),
            &mut io::sink(),
        )?;
        if read != length as u64 {
            return Err(Self::incomplete_read(read as usize));
        }
        Ok(())
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

        Self::read_to_buffer(&mut self.reader, &mut self.temp_buffer, octets)?;

        let is_negative = (self.temp_buffer[0] & 0b1000_0000) > 0;

        self.temp_buffer[0] &= 0b0111_1111;

        let mut number = BigInt::from_bytes_be(Sign::Plus, &self.temp_buffer);

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
        let bytes = self.consume_var_number()?;

        for byte in bytes.iter_mut() {
            *byte &= 0b0111_1111;
        }

        let number = match BigUint::from_radix_be(bytes, 128) {
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
        let bytes = self.consume_var_number()?;

        for byte in bytes.iter_mut() {
            *byte &= 0b0111_1111;
        }

        let is_negative = bytes[0] & 0b0100_0000 > 0;

        bytes[0] &= 0b0011_1111;

        let mut number = match BigInt::from_radix_be(Sign::Plus, bytes, 128) {
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
    fn consume_var_number(&mut self) -> Result<&mut Vec<u8>, ParsingError> {
        self.temp_buffer.clear();
        loop {
            let byte = self.read_byte()?;
            self.temp_buffer.push(byte);
            if byte & 0x80 != 0 {
                break;
            }
        }

        Ok(&mut self.temp_buffer)
    }

    //   7       4 3       0
    //  +---------+---------+
    //  |    T    |    L    |
    //  +---------+---------+
    #[inline]
    pub fn consume_value_header(&mut self, nested_level: u64) -> Result<ValueHeader, ParsingError> {
        loop {
            let byte = self.read_byte()?;
            if byte != 0xE0 {
                return ValueHeader::new(byte);
            }
            if nested_level != 0 {
                return Err(ParsingError::NestedVersionMarker);
            }

            let mut marker = [0; 3];
            match self.read_bytes(&mut marker) {
                Err(ParsingError::NoDataToRead | ParsingError::NotEnoughtDataToRead(_)) => {
                    return Err(ParsingError::BadFormedVersionHeader);
                }
                result => result?,
            }
            match marker {
                [1, 0, 0xEA] => self.symbol_reset_pending = true,
                [major, minor, 0xEA] => return Err(ParsingError::UnsupportedVersion(major, minor)),
                _ => return Err(ParsingError::BadFormedVersionHeader),
            }
        }
    }
}

impl<T: Read> Debug for IonBinaryParser<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.debug_struct("IonBinaryParser").finish()
    }
}
