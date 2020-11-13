use crate::binary_encoder::{encode_datetime_representation, encode_int, encode_varint};
use crate::binary_parser_types::{SystemSymbolIds, SYSTEM_SYMBOL_TABLE};
use crate::{IonValue, NullIonValue};
use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use chrono::{DateTime, FixedOffset};
use digest::Digest;
use num_bigint::{BigInt, Sign};
use std::collections::HashMap;

pub fn encode_value<D: Digest>(value: &IonValue) -> Vec<u8> {
    let encoded_value = match value {
        IonValue::Null(value) => encode_null_value(value),
        IonValue::Bool(value) => encode_bool_value(value),
        IonValue::Integer(value) => encode_integer_value(value),
        IonValue::BigInteger(value) => encode_big_integer_value(value),
        IonValue::Float32(value) => encode_float_value(&f32tof64(value)),
        IonValue::Float64(value) => encode_float_value(value),
        IonValue::Decimal(value) => encode_decimal_value(value),
        IonValue::DateTime(value) => encode_datetime_value(value),
        IonValue::String(value) => encode_string(value, 0x80),
        IonValue::Symbol(value) => encode_symbol(value),
        IonValue::Clob(value) => encode_blob(value, 0x90),
        IonValue::Blob(value) => encode_blob(value, 0xA0),
        IonValue::List(value) => encode_list::<D>(value, 0xB0),
        IonValue::SExpr(value) => encode_list::<D>(value, 0xC0),
        IonValue::Struct(value) => encode_struct::<D>(value),
        IonValue::Annotation(annotations, value) => encode_annotation::<D>(annotations, value),
    };

    add_markers(encoded_value)
}

fn add_markers(mut encoded_value: Vec<u8>) -> Vec<u8> {
    let mut buffer = vec![0x0B];
    buffer.append(&mut encoded_value);
    buffer.push(0x0E);

    buffer
}

fn encode_annotation<D: Digest>(annotations: &[String], value: &IonValue) -> Vec<u8> {
    let mut buffer = vec![0xE0];

    for annotation in annotations {
        buffer.append(&mut encode_value::<D>(&IonValue::Symbol(annotation.into())));
    }

    buffer.append(&mut encode_value::<D>(value));

    buffer
}

fn encode_struct<D: Digest>(values: &HashMap<String, IonValue>) -> Vec<u8> {
    let mut hashes: Vec<Vec<u8>> = vec![];

    for (name, value) in values {
        let mut buffer = add_markers(encode_symbol(name));
        buffer.append(&mut encode_value::<D>(value));
        let hash = D::digest(&buffer).to_vec();
        hashes.push(hash);
    }

    hashes.sort();

    let hashes_buffer: Vec<u8> = hashes.into_iter().flatten().collect();

    let mut buffer: Vec<u8> = escape_buffer(&hashes_buffer);

    let mut header = vec![0xD0];
    header.append(&mut buffer);

    header
}

fn encode_list<D: Digest>(values: &[IonValue], header: u8) -> Vec<u8> {
    let mut buffer = vec![header];

    for value in values {
        buffer.append(&mut encode_value::<D>(value))
    }

    buffer
}

fn encode_blob(value: &[u8], header: u8) -> Vec<u8> {
    let mut buffer = vec![header];

    buffer.append(&mut escape_buffer(&value.to_owned()));

    buffer
}

fn encode_symbol(value: &str) -> Vec<u8> {
    if value == SYSTEM_SYMBOL_TABLE[SystemSymbolIds::Zero as usize] {
        return encode_string(value, 0x71);
    }

    encode_string(value, 0x70)
}

fn encode_string(value: &str, header: u8) -> Vec<u8> {
    let mut buffer = vec![header];

    buffer.append(&mut escape_buffer(&value.as_bytes()));

    buffer
}

fn encode_datetime_value(value: &DateTime<FixedOffset>) -> Vec<u8> {
    let mut buffer = vec![0x60];

    buffer.append(&mut escape_buffer(&encode_datetime_representation(value)));

    buffer
}

// Warning:
//
// BigDecimal doesn't distinguish between -0 and 0, but Ion does, so -0 get
// read as 0 in the Rust implementation. This is ok if Rust never compares
// hashes of the same value with other languages (lets say both Rust and JS
// are parsing a decimal value from the same string).
fn encode_decimal_value(value: &BigDecimal) -> Vec<u8> {
    let mut buffer = vec![0x50];

    if value.is_zero() {
        return buffer;
    }

    let (coefficient, exponent) = value.as_bigint_and_exponent();
    let exponent = -exponent;

    let mut exponent = if exponent.is_zero() {
        vec![0x80]
    } else {
        encode_varint(&exponent.abs().to_be_bytes(), exponent.is_negative())
    };

    let mut coefficient = encode_int(&coefficient);

    let mut representation = vec![];

    representation.append(&mut exponent);
    representation.append(&mut coefficient);

    let mut representation = escape_buffer(&representation);

    buffer.append(&mut representation);

    buffer
}

fn encode_float_value(value: &f64) -> Vec<u8> {
    println!("{:?}", value.to_be_bytes());
    let mut buffer = vec![0x40];

    if value.is_nan() {
        buffer.append(&mut escape_buffer(&[
            0x7F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));
        return buffer;
    }

    if value.is_zero() && value.is_sign_positive() {
        return buffer;
    }

    if value.is_infinite() && value.is_sign_positive() {
        buffer.append(&mut escape_buffer(&[
            0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));
        return buffer;
    }

    if value.is_infinite() && value.is_sign_negative() {
        buffer.append(&mut escape_buffer(&[
            0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));
        return buffer;
    }

    if value.is_zero() && value.is_sign_negative() {
        buffer.append(&mut escape_buffer(&[
            0x80, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));
        return buffer;
    }

    buffer.append(&mut escape_buffer(&value.to_be_bytes().to_vec()));

    buffer
}

fn encode_big_integer_value(value: &BigInt) -> Vec<u8> {
    if value.is_zero() {
        return vec![0x20];
    }

    // It returns the bytes encoded as positive number, same as Ion
    let (sign, bytes) = value.to_bytes_be();

    let mut bytes = escape_buffer(&bytes);

    let mut buffer = if let Sign::Minus = sign {
        vec![0x30]
    } else {
        vec![0x20]
    };

    buffer.append(&mut bytes);

    buffer
}

fn encode_integer_value(value: &i64) -> Vec<u8> {
    encode_big_integer_value(&BigInt::from(*value))
}

fn encode_bool_value(value: &bool) -> Vec<u8> {
    if *value {
        vec![0x11]
    } else {
        vec![0x10]
    }
}

fn encode_null_value(value: &NullIonValue) -> Vec<u8> {
    match value {
        NullIonValue::Null => vec![0x0F],
        NullIonValue::Bool => vec![0x1F],
        NullIonValue::Integer => vec![0x2F],
        NullIonValue::Float => vec![0x4F],
        NullIonValue::Decimal => vec![0x5F],
        NullIonValue::DateTime => vec![0x6F],
        NullIonValue::Symbol => vec![0x7F],
        NullIonValue::String => vec![0x8F],
        NullIonValue::Clob => vec![0x9F],
        NullIonValue::Blob => vec![0xAF],
        NullIonValue::List => vec![0xBF],
        NullIonValue::SExpr => vec![0xCF],
        NullIonValue::Struct => vec![0xDF],
        NullIonValue::Annotation => vec![0xEF],
    }
}

fn escape_buffer(buffer: &[u8]) -> Vec<u8> {
    let mut escaped_buffer = vec![];

    for byte in buffer {
        match byte {
            0x0B | 0x0C | 0x0E => {
                escaped_buffer.push(0x0C);
                escaped_buffer.push(*byte);
            }
            _ => escaped_buffer.push(*byte),
        }
    }

    escaped_buffer
}

// Seems that 123.4f64.to_be_bytes() equals to [64, 94, 217, 153, 153, 153, 153, 154]
// but let value: f32 = 123.4f32; (f64::from(value)).to_be_bytes() equals to
// [64, 94, 217, 153, 160, 0, 0, 0]. Given that I cannot find a way to transform
// the f32 to a f64 in a way that keeps all the bits like if it was declared initially
// as f64, we represent the number in an string and then parse it.
// TODO: Improve transformation method
fn f32tof64(value: &f32) -> f64 {
    value.to_string().parse().unwrap()
}
