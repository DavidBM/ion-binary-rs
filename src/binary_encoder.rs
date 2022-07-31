use crate::NullIonValue;
use bigdecimal::{BigDecimal, Zero};
use chrono::{DateTime, Datelike, FixedOffset, Timelike};
use num_bigint::{BigInt, BigUint, Sign};
use std::convert::TryFrom;

pub const ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED: u8 = 14;
const BITS_IN_BYTE: u8 = 8;

#[cfg(test)]
use crate::IonValue;

#[cfg(test)]
pub fn encode_ion_value(value: &IonValue) -> Vec<u8> {
    match value {
        IonValue::Null(value) => encode_null(value),
        IonValue::Bool(value) => encode_bool(value),
        IonValue::Integer(value) => encode_integer(&BigInt::from(*value)),
        IonValue::BigInteger(value) => encode_integer(value),
        IonValue::Float(value) => encode_float64(value),
        IonValue::Decimal(value) => encode_decimal(value),
        IonValue::String(value) => encode_blob(8, value.as_bytes()),
        IonValue::Clob(value) => encode_blob(9, value),
        IonValue::Blob(value) => encode_blob(10, value),
        IonValue::DateTime(value) => encode_datetime(value),
        _ => unreachable!(),
    }
}

pub fn encode_bool(value: &bool) -> Vec<u8> {
    let mut buffer = vec![];

    encode_bool_buffer(&mut buffer, value).to_vec()
}

pub fn encode_bool_buffer<'a>(buffer: &'a mut Vec<u8>, value: &bool) -> &'a mut Vec<u8> {
    if *value {
        buffer.push(0x11)
    } else {
        buffer.push(0x10)
    }

    buffer
}

pub fn encode_null(value: &NullIonValue) -> Vec<u8> {
    let mut buffer = vec![];

    encode_null_buffer(&mut buffer, value).to_vec()
}

pub fn encode_null_buffer<'a>(buffer: &'a mut Vec<u8>, value: &NullIonValue) -> &'a mut Vec<u8> {
    match value {
        NullIonValue::Null => buffer.push(0x0F),
        NullIonValue::Bool => buffer.push(0x1F),
        NullIonValue::Integer => buffer.push(0x2F),
        NullIonValue::Float => buffer.push(0x4F),
        NullIonValue::Decimal => buffer.push(0x5F),
        NullIonValue::DateTime => buffer.push(0x6F),
        NullIonValue::Symbol => buffer.push(0x7F),
        NullIonValue::String => buffer.push(0x8F),
        NullIonValue::Clob => buffer.push(0x9F),
        NullIonValue::Blob => buffer.push(0xAF),
        NullIonValue::List => buffer.push(0xBF),
        NullIonValue::SExpr => buffer.push(0xCF),
        NullIonValue::Struct => buffer.push(0xDF),
        NullIonValue::Annotation => buffer.push(0xEF),
    }

    buffer
}

pub fn encode_datetime_representation(value: &DateTime<FixedOffset>) -> Vec<u8> {
    let mut buffer = vec![];

    encode_datetime_representation_buffer(&mut buffer, value).to_vec()
}

pub fn encode_datetime_representation_buffer<'a>(buffer: &'a mut Vec<u8>, value: &DateTime<FixedOffset>) -> &'a mut Vec<u8> {
    let datetime = value.naive_utc();

    let year = datetime.year();
    let month = datetime.month();
    let day = datetime.day();
    let hour = datetime.hour();
    let minute = datetime.minute();
    let second = datetime.second();
    let mut nanosecond = datetime.nanosecond();

    // Accounting for the case of a leap second, which shouldn't ever happen.
    // https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveTime.html#leap-second-handling
    if nanosecond > 1_000_000_000 {
        nanosecond -= 1_000_000_000;
    }

    // This gives us a maximum decimal precision of 9 places.
    // It will use less bytes if the number needs less. 23.100 seconds will become 23.1.
    //
    // This means that this implementation is not fully following the Ion Spec.
    // In an Ion Timestamp 23.100 seconds are not the same as 23.1 seconds. An Ion
    // Timestamp comparison between two dates representing the same moment but with
    // different number of zeros in the seconds value results in "not equal". Given
    // that we use DateTime type for the decoded value we loose the original stored
    // precision. We assume that the precision is the lowest one that doesn't
    // loose data. So equality comparisons in this library are less strict than in
    // the Ion standard.
    //
    // Additionally, the ISO standard doesn't caps the maximum quantity of decimals
    // in a seconds, but many implementations do. For example, nodejs rounds to 3
    // decimals, so 23.999 seconds are 23.999 but 23.9999 are 24 seconds.
    //
    // If you are comparing Ion Timestamps and expect the equality to be an Ion
    // equality operation or if you are comparing hashes hashed in Rust and other
    // languages you may end with unexpected results.
    let nanosecond: BigDecimal = BigDecimal::from(nanosecond) / BigDecimal::from(1_000_000_000);

    let (coefficient, exponent) = nanosecond.as_bigint_and_exponent();

    let coefficient = BigInt::from_signed_bytes_le(&coefficient.to_signed_bytes_le());

    let exponent = -exponent;

    let exponent_bytes = exponent.abs().to_be_bytes();
    let exponent_bytes = filter_significant_bytes_slice(&exponent_bytes);

    let offset = value.offset().local_minus_utc() / 60;

    let unsigned_offset = offset.unsigned_abs().to_be_bytes();

    buffer.append(&mut encode_varint(&unsigned_offset, offset.is_negative()));
    buffer.append(&mut encode_varuint(&year.to_be_bytes()));
    buffer.append(&mut encode_varuint(&month.to_be_bytes()));
    buffer.append(&mut encode_varuint(&day.to_be_bytes()));
    buffer.append(&mut encode_varuint(&hour.to_be_bytes()));
    buffer.append(&mut encode_varuint(&minute.to_be_bytes()));
    buffer.append(&mut encode_varuint(&second.to_be_bytes()));

    // Timestamp precision in term of components (day, hour, seconds, etc)
    // depends of the representation.
    // 2011-01-01T00 encodes to 80 0F DB 81 81 80
    // 2011-01-01T00:00:00+00:00 encodes to 80 0F DB 81 81 80 80 80 even
    // if the minutes and seconds are 0.
    // We don't know the original represented precision, so we use seconds
    // or fractional seconds.
    if !exponent.is_zero() && !coefficient.is_zero() {
        buffer.append(&mut encode_varint(
            exponent_bytes,
            exponent.is_negative(),
        ));
        if !coefficient.is_zero() {
            buffer.append(&mut encode_int(&coefficient));
        }
    }

    buffer
}

pub fn encode_datetime(value: &DateTime<FixedOffset>) -> Vec<u8> {
    let mut buffer = encode_datetime_representation(value);

    let len = buffer.len();
    let mut len_bytes = filter_significant_bytes(len.to_be_bytes().to_vec());

    let has_length_field = len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

    if has_length_field {
        len_bytes.append(&mut buffer);
        buffer = len_bytes;
        buffer.insert(0, 0x6E);
    } else {
        buffer.insert(0, u8::try_from(0x60 + len).unwrap());
    }

    buffer
}

pub fn encode_blob(header: u8, value: &[u8]) -> Vec<u8> {
    let mut buffer = vec![];

    encode_blob_buffer(&mut buffer, header, value).to_vec()
}

pub fn encode_blob_buffer<'a>(buffer: &'a mut Vec<u8>, header: u8, value: &[u8]) -> &'a mut Vec<u8> {
    let len = value.len();

    let header = header << 4;

    let has_len_field = len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

    let len_bytes = encode_varuint(&len.to_be_bytes());

    if has_len_field {
        buffer.push(header + ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED);
    } else {
        // Impossible error due to the check of len with 
        // ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED
        buffer.push(header + u8::try_from(len).expect("Impossible error"));
    };

    if has_len_field {
        for value in len_bytes {
            buffer.push(value);
        }
    }

    for value in value {
        buffer.push(*value);
    }

    buffer
}

pub fn encode_decimal(value: &BigDecimal) -> Vec<u8> {
    if *value == BigDecimal::from(0) {
        return vec![0x50];
    }

    let (coefficient, exponent) = value.as_bigint_and_exponent();
    let coefficient = BigInt::from_signed_bytes_le(&coefficient.to_signed_bytes_le());
    let exponent_bytes = filter_significant_bytes(exponent.to_be_bytes().to_vec());
    let mut exponent_bytes = encode_varint(&exponent_bytes, !exponent.is_negative());
    if exponent_bytes.is_empty() {
        // 0x80 = 0 positive in VarInt 0x_1_0_00_0000
        exponent_bytes = vec![0x80];
    }
    let exponent_bytes_len = exponent_bytes.len();
    let coefficient_bytes = encode_int(&coefficient);
    let content_len = exponent_bytes.len() + coefficient_bytes.len();
    let content_len_bytes = encode_varuint(&content_len.to_be_bytes());
    let content_len_bytes_len = content_len_bytes.len();
    let has_len_field = content_len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

    let buffer_len = if has_len_field {
        1 + content_len_bytes_len + content_len
    } else {
        1 + content_len
    };

    let mut buffer: Vec<u8> = vec![0; buffer_len];

    let mut header = 5 << 4;

    if has_len_field {
        header += ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
    } else {
        header += u8::try_from(content_len).expect("Impossible error");
    };

    buffer[0] = header;

    let index_offset = if has_len_field {
        for (index, value) in content_len_bytes.into_iter().enumerate() {
            buffer[index + 1] = value;
        }
        content_len_bytes_len + 1
    } else {
        1
    };

    for (index, value) in exponent_bytes.into_iter().enumerate() {
        buffer[index + index_offset] = value;
    }

    for (index, value) in coefficient_bytes.into_iter().enumerate() {
        buffer[index + index_offset + exponent_bytes_len] = value;
    }

    buffer
}

pub fn encode_int(value: &BigInt) -> Vec<u8> {
    let (_, mut value_bytes) = value.to_bytes_be();

    if *value < BigInt::from(0) {
        if value_bytes[0] & 0b_1000_0000 != 0 {
            value_bytes.insert(0, 0b_1000_0000);
        } else {
            value_bytes[0] |= 0b_1000_0000;
        }
    } else if value_bytes[0] & 0b_1000_0000 != 0 {
        value_bytes.insert(0, 0);
    }

    value_bytes
}

pub fn encode_uint(value: &BigUint) -> Vec<u8> {
    value.to_bytes_be()
}

pub fn encode_float64(value: &f64) -> Vec<u8> {
    if *value == 0.0 && value.is_sign_positive() {
        return vec![0x40];
    }

    let mut buffer: Vec<u8> = vec![0; 9];

    buffer[0] = 0x48;

    let bytes = value.to_be_bytes();

    buffer[1] = bytes[0];
    buffer[2] = bytes[1];
    buffer[3] = bytes[2];
    buffer[4] = bytes[3];
    buffer[5] = bytes[4];
    buffer[6] = bytes[5];
    buffer[7] = bytes[6];
    buffer[8] = bytes[7];

    buffer
}

pub fn encode_integer(value: &BigInt) -> Vec<u8> {
    if *value == BigInt::from(0) {
        return [0x20].to_vec();
    }

    let mut ion_type: u8 = 2;

    let (sign, unsigned_value) = value.to_bytes_be();

    if let Sign::Minus = sign {
        ion_type = 3;
    }

    let bytes = filter_significant_bytes(unsigned_value);
    let bytes_len = bytes.len();
    let bytes_len_bytes = bytes.len().to_be_bytes().to_vec();
    let bytes_len_bytes = filter_significant_bytes(bytes_len_bytes);
    let bytes_len_bytes = encode_varuint(&bytes_len_bytes);
    let bytes_len_bytes_len = bytes_len_bytes.len();
    let has_len_field = bytes_len >= ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED.into();

    let ion_value_len = if has_len_field {
        1 + bytes_len_bytes_len + bytes_len
    } else {
        1 + bytes_len
    };

    let mut result_buffer = vec![0; ion_value_len];

    let mut header = ion_type << 4;

    if has_len_field {
        header += ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED;
    } else {
        header += u8::try_from(bytes_len).expect("Impossible error");
    };

    result_buffer[0] = header;

    let index_offset = if has_len_field {
        for (index, value) in bytes_len_bytes.into_iter().enumerate() {
            result_buffer[index + 1] = value;
        }
        bytes_len_bytes_len + 1
    } else {
        1
    };

    for (index, value) in bytes.into_iter().enumerate() {
        result_buffer[index + index_offset] = value;
    }

    result_buffer
}

pub fn filter_significant_bytes(buffer: Vec<u8>) -> Vec<u8> {
    filter_significant_bytes_slice(&buffer).to_vec()
}

pub fn filter_significant_bytes_reversed_buffer(buffer: &mut Vec<u8>) {

    let mut zeros_count = 0usize;

    for byte in buffer.iter().rev() {
        if *byte != 0u8 {
            break
        }

        zeros_count += 1;
    }

    buffer.resize(buffer.len() - zeros_count, 0);
}

pub fn filter_significant_bytes_slice<'b, 'a: 'b>(buffer: &'a [u8]) -> &'b [u8] {
    let mut zeros_count = 0usize;

    for byte in buffer.iter() {
        if *byte != 0u8 {
            break
        }

        zeros_count += 1;
    }

    let len = buffer.len();

    &buffer[zeros_count..len]
}

pub fn encode_varuint(value: &[u8]) -> Vec<u8> {
    if value.is_empty() {
        return vec![];
    }

    encode_var(value)
}

pub fn encode_varint(value: &[u8], is_negative: bool) -> Vec<u8> {
    if value.is_empty() {
        return vec![];
    }

    let mut buffer = encode_var(value);

    if (buffer[0] & 0b_0100_0000) == 0 {
        if is_negative {
            buffer[0] |= 0b_0100_0000;
        }
    } else {
        buffer.insert(0, 0b_0100_0000)
    }

    buffer
}

pub fn encode_var(value: &[u8]) -> Vec<u8> {
    if value.is_empty() {
        return vec![];
    }

    const RESULTING_BYTE_WIDTH: u8 = 7;

    let mut buffer: Vec<u8> = Vec::with_capacity(value.len()*2);

    let value_len = value.len();
    let mut value_index = value_len - 1;
    let mut remaining_bits = 8;

    loop {
        match remaining_bits {
            7 => {
                let bits = value[value_index] >> 1;
                buffer.push(bits);
                remaining_bits = 0;
            }
            8 => {
                let bits = value[value_index] << 1 >> 1;
                buffer.push(bits);
                remaining_bits = 1;
            }
            0 => {
                if value_index == 0 {
                    break;
                }

                value_index -= 1;
                remaining_bits = BITS_IN_BYTE;
            }
            1..=6 => {
                let shift = BITS_IN_BYTE - remaining_bits;
                let mut buffer_item = value[value_index] >> shift;

                if value_index == 0 {
                    buffer.push(buffer_item);
                    break;
                }

                value_index -= 1;

                let bits_to_take = RESULTING_BYTE_WIDTH - remaining_bits;
                let shift = BITS_IN_BYTE - bits_to_take;

                let bits = value[value_index] << shift >> shift << remaining_bits;

                buffer_item += bits;

                buffer.push(buffer_item);

                remaining_bits = BITS_IN_BYTE - bits_to_take;
            }
            _ => {}
        }
    }

    if let Some(value) = buffer.first_mut() {
        *value += 0b1000_0000;
    }

    filter_significant_bytes_reversed_buffer(&mut buffer);
    
    buffer.reverse();

    buffer
}
