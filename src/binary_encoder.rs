use crate::IonValue;
use crate::NullIonValue;
use num_bigint::{BigInt, Sign};
use std::convert::TryFrom;

const ION_LEN_ON_HEADER_WHEN_EXTRA_LEN_FIELD_REQUIRED: u8 = 14;
const BITS_IN_BYTE: u8 = 8;

pub fn encode_ion_value(value: &IonValue) -> Vec<u8> {
    match value {
        IonValue::Null(NullIonValue::Null) => [0x0F].to_vec(),
        IonValue::Null(NullIonValue::Bool) => [0x1F].to_vec(),
        IonValue::Null(NullIonValue::Integer) => [0x2F].to_vec(),
        IonValue::Null(NullIonValue::BigInteger) => [0x2F].to_vec(),
        IonValue::Null(NullIonValue::Float) => [0x4F].to_vec(),
        IonValue::Null(NullIonValue::Decimal) => [0x5F].to_vec(),
        IonValue::Null(NullIonValue::DateTime) => [0x6F].to_vec(),
        IonValue::Null(NullIonValue::Symbol) => [0x7F].to_vec(),
        IonValue::Null(NullIonValue::String) => [0x8F].to_vec(),
        IonValue::Null(NullIonValue::Clob) => [0x9F].to_vec(),
        IonValue::Null(NullIonValue::Blob) => [0xAF].to_vec(),
        IonValue::Null(NullIonValue::List) => [0xBF].to_vec(),
        IonValue::Null(NullIonValue::SExpr) => [0xCF].to_vec(),
        IonValue::Null(NullIonValue::Struct) => [0xDF].to_vec(),
        IonValue::Null(NullIonValue::Annotation) => [0xEF].to_vec(),
        IonValue::Bool(value) => {
            if *value {
                [0x11].to_vec()
            } else {
                [0x10].to_vec()
            }
        }
        IonValue::Integer(value) => encode_integer(&BigInt::from(*value)),
        IonValue::BigInteger(value) => encode_integer(value),
        //IonValue::Float32(f32),
        //IonValue::Float64(f64),
        //IonValue::Decimal(BigDecimal),
        //IonValue::DateTime(DateTime<FixedOffset>),
        //IonValue::String(String),
        //IonValue::Symbol(String),
        //IonValue::Clob(Vec<u8>),
        //IonValue::Blob(Vec<u8>),
        _ => vec![],
    }
}

fn encode_integer(value: &BigInt) -> Vec<u8> {
    if *value == BigInt::from(0) {
        return [0x20].to_vec();
    }

    let mut ion_type: u8 = 2;

    let (sign, unsigned_value) = value.to_bytes_be();

    if let Sign::Minus = sign {
        ion_type = 3;
    }

    let bytes = filter_significant_bytes(&unsigned_value);
    let bytes_len = bytes.len();
    let bytes_len_bytes = bytes.len().to_be_bytes();
    let bytes_len_bytes = filter_significant_bytes(&bytes_len_bytes);
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
        2
    } else {
        1
    };

    for (index, value) in bytes.into_iter().enumerate() {
        result_buffer[index + index_offset] = value;
    }

    result_buffer
}

fn filter_significant_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut buffer = vec![];

    let mut found_not_zero = false;

    for byte in bytes {
        if *byte != 0u8 {
            found_not_zero = true;
        }

        if found_not_zero {
            buffer.push(*byte);
        }
    }

    buffer
}

fn encode_varuint(value: &[u8]) -> Vec<u8> {
    consume_var(value, 8)
}

fn encode_varint(value: &[u8], is_negative: bool) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut bits = value[0] << 2 >> 2;

    if is_negative {
        bits += 0b_0100_0000;
    }

    buffer.push(bits);

    consume_var(&value[1..], 2)
}

fn consume_var(value: &[u8], remaining_bits: u8) -> Vec<u8> {
    const RESULTING_BYTE_WIDTH: u8 = 7;

    let mut buffer: Vec<u8> = Vec::new();

    let value_len = value.len();
    let mut value_index = 0;
    let mut remaining_bits = remaining_bits;

    loop {
        match remaining_bits {
            7 => {
                let bits = value[value_index] << 1 >> 1;
                buffer.push(bits);
                remaining_bits = 1;
            }
            8 => {
                let bits = value[value_index] >> 1;
                buffer.push(bits);
                remaining_bits = 0;
            }
            0 => {
                value_index += 1;
                remaining_bits = BITS_IN_BYTE;
            }
            1..=6 => {
                let shift = BITS_IN_BYTE - remaining_bits;
                let bits = value[value_index] << shift >> shift;

                //We substract one ecause a byte has 8 bits and our resulting byte
                //has only 7 significant bits
                let mut buffer_item = bits << (shift - 1);

                let bites_to_take = RESULTING_BYTE_WIDTH - remaining_bits;
                value_index += 1;
                remaining_bits = BITS_IN_BYTE - bites_to_take;

                let bits = value[value_index] >> remaining_bits;

                buffer_item += bits;

                buffer.push(buffer_item)
            }
            _ => {}
        }

        if value_index >= value_len {
            break;
        }
    }

    if let Some(value) = buffer.last_mut() {
        *value += 0b1000_0000;
    }

    filter_significant_bytes(&buffer)
}
